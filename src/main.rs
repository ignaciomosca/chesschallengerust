use chess::{solution, Board, ChessPiece, Piece};
use std::collections::BTreeSet;
use std::collections::HashSet;

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
    let solutions = solution(board, pieces, &mut HashSet::new(), &mut HashSet::new());
    println!("Size {}", solutions.len()); // it has to be 3063828
}
