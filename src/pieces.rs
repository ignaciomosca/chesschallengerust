
use std::collections::{HashSet};
use std::vec::Vec;

    #[derive(PartialEq, Eq, Hash)]
    pub enum ChessPiece { Rook , Bishop , Knight , Queen , King }

    #[derive(PartialEq, Eq, Hash)]
    pub struct Piece {
       pub row: i8, 
       pub col: i8,
       pub piece: ChessPiece
    }

    pub struct Board {
        pub m: i8, 
        pub n: i8, 
        pub used_pieces: HashSet<Piece>, 
        pub number_of_pieces: i8
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
                ChessPiece::Bishop => i8::abs(self.row - chess_piece.row) == i8::abs(self.col - chess_piece.col),
                ChessPiece::Knight => Self::knight_moves(self, chess_piece),
                ChessPiece::Queen => self.row == chess_piece.row || self.col == chess_piece.col || i8::abs(self.row - chess_piece.row) == i8::abs(self.col - chess_piece.col),
                ChessPiece::King => Self::king_moves(self, chess_piece)
            }
        }

    }

    impl Board {

        pub fn new(m: i8, n: i8, used_pieces: HashSet<Piece>, number_of_pieces: i8) -> Board {
            return Board { m: m, n: n, used_pieces: used_pieces, number_of_pieces: number_of_pieces };
        }

        pub fn is_safe(&self, chess_piece: Piece) -> bool {
            self.used_pieces
            .iter()
            .any(|piece| !piece.attacks(&chess_piece) && !chess_piece.attacks(&piece))
        }

        pub fn done(&self) -> bool {
            return self.used_pieces.len() == self.number_of_pieces as usize;
        }

        // pub fn place(&self, chess_piece: Piece) -> Board {
        //     self.used_pieces.insert(chess_piece);
        //     return Board::new(self.m, self.n, self.used_pieces, self.number_of_pieces);
        // }

    
    }

#[cfg(test)]
mod test;


