mod pieces;
use std::collections::HashSet;
use pieces::{solution, Board, ChessPiece, Piece};
use im::hashset::HashSet as ImmutableHashSet;

fn main() {
    let board = Board {
        m: 7,
        n: 7,
        used_pieces: ImmutableHashSet::new()
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
    println!("Size {}", solutions.len());
}
