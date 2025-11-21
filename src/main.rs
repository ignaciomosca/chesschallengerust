use chess::{solution, Board, ChessPiece};
use std::{collections::HashSet, collections::VecDeque};
use std::rc::Rc;

fn main() {
    let board = Board {
        rows: 7,
        cols: 7,
        pieces: Vec::new(),
    };
    let board_rc = Rc::new(board);
    let pieces = [
        ChessPiece::King,
        ChessPiece::King,
        ChessPiece::Queen,
        ChessPiece::Queen,
        ChessPiece::Bishop,
        ChessPiece::Bishop,
        ChessPiece::Knight,
    ];
    let mut solutions: HashSet<Rc<Board>> = HashSet::new();
    let mut board_stack: VecDeque<(Rc<Board>, &[ChessPiece])> = VecDeque::new();
    board_stack.push_front((board_rc, &pieces));
    solution(&mut board_stack, &mut solutions, &mut HashSet::new());
    println!("Size {}", solutions.len()); // it has to be 3063828


}
