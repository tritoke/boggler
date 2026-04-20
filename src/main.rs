use std::{collections::BTreeSet, hint::unreachable_unchecked};

use rayon::prelude::*;
use trie_rs::{Trie, try_collect::{TryFromIterator, Collect}};

mod board;
use board::{BoardPath, Boggle, Tile::*};

use crate::board::{ALL_TILES, Tile, TileLocation};

static TRIE_POSTCARD: &[u8] = include_bytes!(concat!(std::env!("OUT_DIR"), "/trie.postcard"));

fn main() {
    let trie: Trie<u8> = postcard::from_bytes(TRIE_POSTCARD).unwrap();

    #[rustfmt::skip]
    let board = Boggle {
        tiles: [
            [D, W, G, H],
            [R, L, N, E],
            [U, O, T, A],
            [S, I, C, M],
        ]
    };

    println!("{board}");

    let mut all_words = vec![];
    let par_solver = ALL_TILES.into_par_iter().flat_map(|tile| solve_from_tile(&board, &trie, tile));
    all_words.par_extend(par_solver);

    all_words.sort_by_key(|word| 16 - word.len());
    for word in all_words {
        let score = match word.len() {
            // SAFETY: the trie only contains words longer than three characters
            0 | 1 | 2 => unsafe { unreachable_unchecked() },
            3 | 4 => 1,
            5 => 2,
            6 => 3,
            7 => 5,
            _ => 11,
        };

        println!("{word}: {score}");
    }
}

fn solve_from_tile(board: &Boggle, trie: &Trie<u8>, starting_tile: TileLocation) -> Vec<String> {
    let mut words = vec![];

    let mut to_explore = vec![BoardPath::starting_at(starting_tile)];
    let mut word = String::with_capacity(32);
    let mut valid_next_chars = BTreeSet::new();
    while let Some(prior_path) = to_explore.pop() {
        word.clear();

        // construct the word this path represents
        for tile in prior_path {
            word.push_str(board[tile].as_str());
        }

        // if its in the dictionary the add it as a found word
        if trie.exact_match(&word) {
            words.push(word.clone());
        }

        // path can only store 15 elements and this case is trivial so handle it here
        if prior_path.len() == 15 {
            let Some(tile) = prior_path.valid_next_tiles().next() else {
                // if there is no next tile then this path has ended
                continue;
            };

            word.push_str(board[tile].as_str());
            if trie.exact_match(&word) {
                words.push(word.clone());
            }

            // nothing else to do
            continue;
        }

        valid_next_chars.clear();
        valid_next_chars.extend(trie.postfix_search(&word).filter_map(|c: CollectOne| c.0));

        if valid_next_chars.is_empty() {
            continue;
        }

        // if the path is less than 15 characters long push all of its valid continuations to the stack
        for tile in prior_path.valid_next_tiles() {
            if !valid_next_chars.contains(&board[tile]) {
                continue
            }

            let mut next = prior_path;
            next.push(tile);
            to_explore.push(next);
        }
    }

    words
}

struct CollectOne(Option<Tile>);

impl TryFromIterator<u8, Collect> for CollectOne {
    type Error = ();

    fn try_from_iter<T>(iter: T) -> Result<Self, Self::Error>
        where
            Self: Sized,
            T: IntoIterator<Item = u8> {

        let mut iter = iter.into_iter();
        let Some(c) = iter.next() else {
            return Ok(Self(None));
        };

        if c == b'Q' && iter.next() != Some(b'U') {
            return Ok(Self(None));
        }

        // fucking better be
        debug_assert!(b'A' <= c && c <= b'Z');

        // SAFETY: the trie only contains these characters
        Ok(Self(unsafe { std::mem::transmute(c) }))
    }
}
