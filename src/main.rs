use chess::{solution, Board, ChessPiece};
use std::{collections::HashSet, collections::VecDeque};

fn main() {
    let board = Board {
        rows: 7,
        cols: 7,
        pieces: Vec::new(),
    };
    let pieces = [
        ChessPiece::King,
        ChessPiece::King,
        ChessPiece::Queen,
        ChessPiece::Queen,
        ChessPiece::Bishop,
        ChessPiece::Bishop,
        ChessPiece::Knight,
    ];
    let mut solutions: HashSet<Board> = HashSet::new();
    let mut board_stack: VecDeque<(Board, &[ChessPiece])> = VecDeque::new();
    board_stack.push_front((board, &pieces));
    solution(&mut board_stack, &mut solutions, &mut HashSet::new());
    println!("Size {}", solutions.len()); // it has to be 3063828


}
