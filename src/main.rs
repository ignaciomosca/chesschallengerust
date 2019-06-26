use chess::{solution, Board, ChessPiece};
use std::{collections::HashSet, collections::VecDeque};

fn main() {
    let board = Board {
        m: 7,
        n: 7,
        used_pieces: Vec::new(),
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
    let mut board_stack: VecDeque<(Board, Vec<ChessPiece>)> = VecDeque::new();
    board_stack.push_front((board,pieces));
    solution(&mut board_stack, &mut solutions, &mut HashSet::new());
    println!("Size {}", solutions.len()); // it has to be 3063828


}
