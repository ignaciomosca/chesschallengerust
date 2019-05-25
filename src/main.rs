
mod pieces;
use pieces::{Piece, ChessPiece};

fn main() {
    let piece = Piece { row: 1, col: 1, piece: ChessPiece::King };
    println!("Hello, world!");
}
