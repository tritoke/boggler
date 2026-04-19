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
}

fn solve_from_tile(board: &Boggle, trie: &Trie<u8>, starting_tile: (usize, usize)) -> Vec<String> {
    let mut words = vec![];

    let tile_loc = TileLocation::try_from(starting_tile).unwrap();
    let mut to_explore = vec![BoardPath::starting_at(tile_loc)];
    let mut curr_word = String::with_capacity(32);
    while !to_explore.is_empty() {
        todo!()
    }

    words
}
