use im::hashset::HashSet;
use std::collections::HashSet as MutableHashSet;
use std::iter::FromIterator;
use std::vec::Vec;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ChessPiece {
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Piece {
    pub row: i8,
    pub col: i8,
    pub piece: ChessPiece,
}
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Board {
    pub m: i8,
    pub n: i8,
    pub used_pieces: HashSet<Piece>,
    pub number_of_pieces: i8,
}

impl Piece {
    pub fn knight_moves(&self, chess_piece: &super::Piece) -> bool {
        let x_moves = vec![1, 2, 2, 1, -1, -2, -2, -1];
        let y_moves = vec![-2, -1, 1, 2, 2, 1, -1, -2];
        for i in 0..8 {
            let dest_row = self.row + x_moves[i];
            let dest_col = self.col + y_moves[i];
            if dest_row == chess_piece.row && dest_col == chess_piece.col {
                return true;
            }
        }
        return false;
    }

    pub fn king_moves(&self, chess_piece: &super::Piece) -> bool {
        let x_moves = vec![-1, -1, -1, 0, 1, 1, 1, 0];
        let y_moves = vec![-1, 0, 1, 1, 1, 0, -1, -1];
        for i in 0..8 {
            let dest_row = self.row + x_moves[i];
            let dest_col = self.col + y_moves[i];
            if dest_row == chess_piece.row && dest_col == chess_piece.col {
                return true;
            }
        }
        return false;
    }

    pub fn attacks(&self, chess_piece: &super::Piece) -> bool {
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
    pub fn new(m: i8, n: i8, used_pieces: HashSet<Piece>, number_of_pieces: i8) -> Board {
        return Board {
            m,
            n,
            used_pieces,
            number_of_pieces,
        };
    }

    pub fn is_safe(self, chess_piece: Piece) -> bool {
        self.used_pieces
            .iter()
            .any(|piece| !piece.attacks(&chess_piece) && !chess_piece.attacks(&piece))
    }

    pub fn done(&self) -> bool {
        return self.used_pieces.len() == self.number_of_pieces as usize;
    }

    pub fn place(self, chess_piece: Piece) -> Board {
        let updated_pieces = self.used_pieces.update(chess_piece);
        return Board::new(self.m, self.n, updated_pieces, self.number_of_pieces);
    }
}

pub fn find_candidate(board: Board, piece: ChessPiece) -> HashSet<Board> {
    println!("Hello {}", 1);
    let mut result = MutableHashSet::new();
    for row in 1..7 {
        for col in 1..7 {
            let piece = Piece {
                row,
                col,
                piece,
            };
            if board.clone().is_safe(piece) {
                result.insert(board.clone().place(piece));
            }
        }
    }
    return HashSet::from(result);
}

pub fn solution(
    board: Board,
    pieces: Vec<ChessPiece>,
    mut solutions: HashSet<Board>,
    mut tested_configurations: HashSet<Board>,
) -> HashSet<Board> {
    if !pieces.is_empty() {
        for row in 1..7 {
            for col in 1..7 {
                let new_piece = Piece{ row, col, piece: pieces[0] };
                if board.is_safe(new_piece) {
                    let new_board = board.place(new_piece);
                    if pieces.len() != 1 {
                        if tested_configurations.contains(&new_board) {
                            let tail = Vec::from_iter(pieces[1..pieces.len()].iter().cloned());
                            tested_configurations.insert(new_board);
                            solution(new_board, tail, solutions, tested_configurations);
                        }
                    } else {
                        if !solutions.contains(&new_board) {
                            solutions.insert(new_board);
                        }

                    }
                }

            }
        }

    }
    return solutions;
}

#[cfg(test)]
mod test;
