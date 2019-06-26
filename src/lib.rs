use std::{collections::HashSet, collections::VecDeque, iter::FromIterator, vec::Vec};

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
pub enum ChessPiece {
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
pub struct Piece {
    pub row: i8,
    pub col: i8,
    pub piece: ChessPiece,
}
#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd)]
pub struct Board {
    pub m: i8,
    pub n: i8,
    pub used_pieces: Vec<Piece>,
}

impl Piece {
    fn knight_moves(self, chess_piece: Piece) -> bool {
        const KNIGHT_MOVES: [(i8, i8); 9] = [
            (1, -2),
            (2, -1),
            (2, 1),
            (1, 2),
            (0, 0),
            (-1, 2),
            (-2, 1),
            (-2, -1),
            (-1, -2),
        ];
        KNIGHT_MOVES.iter().any(|(x_offset, y_offset)| {
            let dest_row = self.row + x_offset;
            let dest_col = self.col + y_offset;
            dest_row == chess_piece.row && dest_col == chess_piece.col
        })
    }

    fn king_moves(self, chess_piece: Piece) -> bool {
        const KING_MOVES: [(i8, i8); 9] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        KING_MOVES.iter().any(|(x_offset, y_offset)| {
            let dest_row = self.row + x_offset;
            let dest_col = self.col + y_offset;
            dest_row == chess_piece.row && dest_col == chess_piece.col
        })
    }

    pub fn attacks(self, chess_piece: Piece) -> bool {
        match self.piece {
            ChessPiece::Rook => self.row == chess_piece.row || self.col == chess_piece.col,
            ChessPiece::Bishop => {
                i8::abs(self.row - chess_piece.row) == i8::abs(self.col - chess_piece.col)
            }
            ChessPiece::Knight => Self::knight_moves(self, chess_piece),
            ChessPiece::Queen => {
                self.row == chess_piece.row
                    || self.col == chess_piece.col
                    || i8::abs(self.row - chess_piece.row) == i8::abs(self.col - chess_piece.col)
            }
            ChessPiece::King => Self::king_moves(self, chess_piece),
        }
    }
}

impl Board {
    pub fn new(m: i8, n: i8, used_pieces: Vec<Piece>) -> Board {
        Board { m, n, used_pieces }
    }

    pub fn is_safe(&self, chess_piece: Piece) -> bool {
        self.used_pieces
            .iter()
            .all(|&piece| !piece.attacks(chess_piece) && !chess_piece.attacks(piece))
    }

    pub fn place(&self, chess_piece: Piece) -> Board {
        let mut updated_pieces = self.used_pieces.clone();
        if !updated_pieces.contains(&chess_piece) {
            updated_pieces.push(chess_piece);
            updated_pieces.sort();
        }
        Board::new(self.m, self.n, updated_pieces)
    }
}

pub fn solution<'a>(
    board_stack: &mut VecDeque<(Board, Vec<ChessPiece>)>,
    solutions: &'a mut HashSet<Board>,
    tested_configurations: &mut HashSet<Board>,
) -> &'a HashSet<Board> {
    while !board_stack.is_empty() {
        let (board, pieces) = board_stack.pop_back().unwrap();
        for row in 1..=board.m {
            for col in 1..=board.n {
                let new_piece = Piece {
                    row,
                    col,
                    piece: pieces[0],
                };
                if board.is_safe(new_piece) {
                    let new_board = board.place(new_piece);
                    if pieces.len() != 1 {
                        if tested_configurations.insert(new_board.clone()) {
                            let next_board = new_board.clone();
                            let tail = Vec::from_iter(pieces[1..pieces.len()].iter().cloned());
                            board_stack.push_front((next_board, tail));
                        }
                    } else {
                        solutions.insert(new_board);
                    }
                }
            }
        }
    }
    solutions
}
