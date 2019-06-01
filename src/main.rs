
mod pieces;
use pieces::{Board, ChessPiece, Piece, solution};
use im::hashset::HashSet;

fn main() {
    let board = Board {m: 7, n: 7, used_pieces: HashSet::new(), number_of_pieces: 7};
    let pieces = vec![ChessPiece::King,ChessPiece::King,ChessPiece::Queen, ChessPiece::Queen, ChessPiece::Bishop, ChessPiece::Bishop, ChessPiece::Knight];
    let solutions = solution(board, pieces, HashSet::new());
    println!("Size {}", solutions.len());
}
