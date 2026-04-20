use trie_rs::Trie;

mod board;
use board::{BoardPath, Boggle, Tile::*};

use crate::board::TileLocation;

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

    dbg!(solve_from_tile(&board, &trie, (2, 0)));
}

fn solve_from_tile(board: &Boggle, trie: &Trie<u8>, starting_tile: (usize, usize)) -> Vec<String> {
    let mut words = vec![];

    let tile_loc = TileLocation::try_from(starting_tile).unwrap();
    let mut to_explore = vec![BoardPath::starting_at(tile_loc)];
    let mut word = String::with_capacity(32);
    while let Some(prior_path) = to_explore.pop() {
        // dbg!("hi");
        // dbg!(prior_path.into_iter().collect::<Vec<_>>());
        word.clear();

        // construct the word this path represents
        for tile in prior_path {
            word.push_str(board[tile].as_str());
        }

        // if its in the dictionary the add it as a found word
        if trie.exact_match(&word) {
            dbg!(&word);
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
                dbg!(&word);
                words.push(word.clone());
            }

            // nothing else to do
            continue;
        }

        // if the path is less than 15 characters long push all of its valid continuations to the stack
        for tile in prior_path.valid_next_tiles() {
            // TODO: use the trie to consider whether the next letter is even worth exploring?
            let mut next = prior_path;
            next.push(tile);
            to_explore.push(next);
        }
    }

    words
}
