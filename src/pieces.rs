use std::{
    collections::HashSet,
    iter::FromIterator,
    vec::Vec,
};
use im::hashset::HashSet as ImmutableHashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ChessPiece {
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Piece {
    pub row: i8,
    pub col: i8,
    pub piece: ChessPiece,
}
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Board {
    pub m: i8,
    pub n: i8,
    pub used_pieces: ImmutableHashSet<Piece>
}

impl Piece {
    pub fn knight_moves(&self, chess_piece: super::Piece) -> bool {
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

    pub fn king_moves(&self, chess_piece: super::Piece) -> bool {
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

    pub fn attacks(&self, chess_piece: super::Piece) -> bool {
        match self.piece {
            ChessPiece::Rook => self.row == chess_piece.row || self.col == chess_piece.col,
            ChessPiece::Bishop => {
                i8::abs(self.row - chess_piece.row) == i8::abs(self.col - chess_piece.col)
            }
            ChessPiece::Knight => Self::knight_moves(&self, chess_piece),
            ChessPiece::Queen => {
                self.row == chess_piece.row
                    || self.col == chess_piece.col
                    || i8::abs(self.row - chess_piece.row) == i8::abs(self.col - chess_piece.col)
            }
            ChessPiece::King => Self::king_moves(&self, chess_piece),
        }
    }
}

impl Board {
    pub fn new(m: i8, n: i8, used_pieces: ImmutableHashSet<Piece>) -> Board {
        return Board {
            m,
            n,
            used_pieces,
        };
    }

    pub fn is_safe(self, chess_piece: Piece) -> bool {
        self.used_pieces
            .iter()
            .all(|&piece| !piece.attacks(chess_piece) && !chess_piece.attacks(piece))
    }

    pub fn place(&self, chess_piece: Piece) -> Board {
        let updated_pieces = self.used_pieces.update(chess_piece);
        return Board::new(self.m, self.n, updated_pieces);
    }
}

pub fn solution(
    board: Board,
    pieces: Vec<ChessPiece>,
    solutions: &mut HashSet<Board>,
    tested_configurations: &mut HashSet<Board>,
) -> HashSet<Board> {
    if !pieces.is_empty() {
        for row in 1..=board.m {
            for col in 1..=board.n {
                let new_piece = Piece{ row, col, piece: pieces[0] };
                if board.clone().is_safe(new_piece) {
                    let new_board = board.clone().place(new_piece);
                    if pieces.len() != 1 {
                        let contain_board = Board::new(new_board.m, new_board.n, new_board.used_pieces.clone());
                        if !tested_configurations.contains(&contain_board) {
                            let tail = Vec::from_iter(pieces[1..pieces.len()].iter().cloned());
                            tested_configurations.insert(contain_board);
                            solution(new_board, tail, solutions, tested_configurations);
                        }
                    } else {
                        let a_board_pieces = new_board.used_pieces.clone();                    
                        let a_board = Board::new(board.m, board.n, a_board_pieces);
                        if !solutions.contains(&a_board) {
                            solutions.insert(a_board);
                        }

                    }
                }

            }
        }
    }
    println!("solutions {:?}", solutions);
    return solutions.clone();
}

#[cfg(test)]
mod test;
