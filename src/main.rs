use chess::{solution, Board, ChessPiece};
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let board = Board {
        m: 7,
        n: 7,
        used_pieces: BTreeSet::new(),
    };
    let pieces = vec![
        ChessPiece::King,
        ChessPiece::King,
        ChessPiece::Queen,
        ChessPiece::Queen,
        ChessPiece::Bishop,
        ChessPiece::Bishop,
        ChessPiece::Knight,
    ];
    let mut solutions: HashSet<Board> = HashSet::new();
    solution(board, pieces, &mut solutions, &mut HashSet::new());
    println!("Size {}", solutions.len()); // it has to be 3063828


}
