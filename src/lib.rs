//! Chess puzzle solver library.
//!
//! This library provides tools for solving the chess challenge problem: finding all unique
//! configurations of chess pieces on an M×N board where no piece can attack any other piece.
//!
//! # Example
//!
//! ```
//! use chess::{solution, Board, ChessPiece};
//! use std::collections::{HashSet, VecDeque};
//! use std::rc::Rc;
//!
//! let board = Board {
//!     rows: 3,
//!     cols: 3,
//!     pieces: Vec::new(),
//! };
//! let pieces = [ChessPiece::King, ChessPiece::King, ChessPiece::Rook];
//! let mut solutions: HashSet<Rc<Board>> = HashSet::new();
//! let mut board_stack: VecDeque<(Rc<Board>, &[ChessPiece])> = VecDeque::new();
//! board_stack.push_front((Rc::new(board), &pieces));
//! solution(&mut board_stack, &mut solutions, &mut HashSet::new());
//! println!("Found {} solutions", solutions.len());
//! ```

use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

/// Represents the different types of chess pieces used in the puzzle.
///
/// Each piece type has unique movement and attack patterns:
/// - `Rook`: Attacks horizontally and vertically
/// - `Bishop`: Attacks diagonally
/// - `Knight`: Attacks in an L-shape (2 squares in one direction, 1 in perpendicular)
/// - `Queen`: Attacks horizontally, vertically, and diagonally
/// - `King`: Attacks all adjacent squares (one square in any direction)
#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
pub enum ChessPiece {
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

/// Represents a chess piece placed on a specific position on the board.
///
/// Coordinates use a **1-indexed system** where (1,1) represents the top-left
/// corner of the board. A piece at position (row, col) means it's at the `row`-th
/// row and `col`-th column.
///
/// # Fields
///
/// * `row` - The row position (1-indexed, 1 ≤ row ≤ board_rows)
/// * `col` - The column position (1-indexed, 1 ≤ col ≤ board_cols)
/// * `piece` - The type of chess piece (King, Queen, Rook, Bishop, or Knight)
#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
pub struct Piece {
    pub row: i8,
    pub col: i8,
    pub piece: ChessPiece,
}
/// Represents a chess board configuration with its dimensions and placed pieces.
///
/// A board tracks its size (rows × cols) and all pieces currently placed on it.
/// Boards are used to explore different configurations during the search for valid
/// solutions where no pieces attack each other.
///
/// # Fields
///
/// * `rows` - The number of rows on the board (height)
/// * `cols` - The number of columns on the board (width)
/// * `used_pieces` - Vector of all pieces currently placed on this board
///
/// # Note
///
/// Boards use 1-indexed coordinates, so valid positions range from (1,1) to (rows,cols).
#[derive(PartialEq, Eq, Clone, Hash, Debug, PartialOrd)]
pub struct Board {
    pub rows: i8,
    pub cols: i8,
    pub pieces: Vec<Piece>,
}

impl Piece {
    fn knight_moves(self, chess_piece: Piece) -> bool {
        const KNIGHT_MOVES: [(i8, i8); 9] = [
            (1, -2),
            (2, -1),
            (2, 1),
            (1, 2),
            (0,0),
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
            (0, 1),
            (0,0),
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

    /// Determines whether this piece can attack another piece based on chess movement rules.
    ///
    /// Each piece type has different attack patterns:
    /// - **Rook**: Can attack any piece in the same row or column
    /// - **Bishop**: Can attack any piece on the same diagonal
    /// - **Knight**: Can attack pieces that are 2 squares away in one direction and 1 square perpendicular
    /// - **Queen**: Combines Rook and Bishop (same row, column, or diagonal)
    /// - **King**: Can attack any piece in the 8 adjacent squares
    ///
    /// # Arguments
    ///
    /// * `chess_piece` - The target piece to check if it can be attacked
    ///
    /// # Returns
    ///
    /// `true` if this piece can attack the target piece, `false` otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use chess::{Piece, ChessPiece};
    ///
    /// let rook = Piece { row: 1, col: 1, piece: ChessPiece::Rook };
    /// let target = Piece { row: 1, col: 5, piece: ChessPiece::King };
    /// assert!(rook.attacks(target)); // Rook attacks along the same row
    /// ```
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
    /// Creates a new chess board with the specified dimensions and pieces.
    ///
    /// # Arguments
    ///
    /// * `rows` - The number of rows (height) of the board
    /// * `cols` - The number of columns (width) of the board
    /// * `used_pieces` - Vector of pieces already placed on the board
    ///
    /// # Returns
    ///
    /// A new `Board` instance with the given configuration
    ///
    /// # Example
    ///
    /// ```
    /// use chess::Board;
    ///
    /// let board = Board::new(8, 8, Vec::new());
    /// assert_eq!(board.rows, 8);
    /// assert_eq!(board.cols, 8);
    /// ```
    pub fn new(rows: i8, cols: i8, used_pieces: Vec<Piece>) -> Board {
        Board { rows, cols, pieces: used_pieces }
    }

    /// Checks whether a piece can be safely placed without attacking or being attacked.
    ///
    /// A position is considered safe if the piece to be placed neither attacks any existing
    /// piece on the board, nor is attacked by any existing piece.
    ///
    /// # Arguments
    ///
    /// * `chess_piece` - The piece to check for safe placement
    ///
    /// # Returns
    ///
    /// `true` if the piece can be safely placed, `false` if it would attack or be attacked
    ///
    /// # Example
    ///
    /// ```
    /// use chess::{Board, Piece, ChessPiece};
    ///
    /// let board = Board::new(3, 3, Vec::new());
    /// let piece = Piece { row: 1, col: 1, piece: ChessPiece::Rook };
    /// assert!(board.is_safe(piece)); // Safe on an empty board
    /// ```
    pub fn is_safe(&self, chess_piece: Piece) -> bool {
        self.pieces
            .iter()
            .all(|&piece| !piece.attacks(chess_piece) && !chess_piece.attacks(piece))
    }

    /// Creates a new board with the given piece added to it.
    ///
    /// This method returns a new `Board` instance with the piece added to the list of
    /// placed pieces. If the piece already exists at that exact position, it won't be
    /// added again. The pieces are kept sorted to ensure consistent board comparisons.
    ///
    /// # Arguments
    ///
    /// * `chess_piece` - The piece to place on the board
    ///
    /// # Returns
    ///
    /// A new `Board` with the piece added (or the same pieces if it was already present)
    ///
    /// # Note
    ///
    /// This method does not check if the placement is safe - use `is_safe()` first
    /// if you need to verify the piece won't attack or be attacked by existing pieces.
    ///
    /// # Example
    ///
    /// ```
    /// use chess::{Board, Piece, ChessPiece};
    ///
    /// let board = Board::new(3, 3, Vec::new());
    /// let piece = Piece { row: 1, col: 1, piece: ChessPiece::King };
    /// let new_board = board.place(piece);
    /// assert_eq!(new_board.pieces.len(), 1);
    /// ```
    pub fn place(&self, chess_piece: Piece) -> Board {
        let mut updated_pieces = self.pieces.clone();
        if !updated_pieces.contains(&chess_piece) {
            updated_pieces.push(chess_piece);
            updated_pieces.sort();
        }
        Board::new(self.rows, self.cols, updated_pieces)
    }
}

/// Finds all unique board configurations where chess pieces don't threaten each other.
///
/// This function uses a backtracking algorithm to systematically explore all possible
/// placements of the given pieces on the board. It iterates through each board position
/// (using 1-indexed coordinates from 1 to rows/cols) and attempts to place pieces where
/// they won't attack or be attacked by existing pieces.
///
/// The algorithm:
/// 1. Takes a board state and remaining pieces to place from the work queue
/// 2. Tries placing the next piece at every position on the board
/// 3. If a placement is safe and valid:
///    - If more pieces remain, adds the new configuration to the work queue
///    - If this was the last piece, adds the complete configuration to solutions
/// 4. Uses `tested_configurations` to avoid exploring duplicate board states
///
/// # Arguments
///
/// * `board_stack` - A work queue of (Board, remaining pieces) tuples to explore.
///   Initially should contain one entry with an empty board and all pieces.
/// * `solutions` - Accumulator for all valid complete board configurations found.
///   Will be populated with boards where all pieces are placed safely.
/// * `tested_configurations` - Cache of intermediate board states already explored,
///   used to prune duplicate branches in the search space.
///
/// # Returns
///
/// A reference to the `solutions` HashSet containing all valid configurations.
///
/// # Panics
///
/// This function will panic if `board_stack` becomes empty during iteration,
/// though this should not happen given the while loop condition.
///
/// # Example
///
/// ```
/// use chess::{solution, Board, ChessPiece};
/// use std::collections::{HashSet, VecDeque};
/// use std::rc::Rc;
///
/// let board = Board {
///     rows: 3,
///     cols: 3,
///     pieces: Vec::new(),
/// };
/// let pieces = [ChessPiece::King, ChessPiece::King, ChessPiece::Rook];
/// let mut solutions: HashSet<Rc<Board>> = HashSet::new();
/// let mut board_stack: VecDeque<(Rc<Board>, &[ChessPiece])> = VecDeque::new();
/// board_stack.push_front((Rc::new(board), &pieces));
///
/// solution(&mut board_stack, &mut solutions, &mut HashSet::new());
/// assert_eq!(solutions.len(), 4); // 4 valid configurations for this setup
/// ```
pub fn solution<'a>(
    board_stack: &mut VecDeque<(Rc<Board>, &[ChessPiece])>,
    solutions: &'a mut HashSet<Rc<Board>>,
    tested_configurations: &mut HashSet<Rc<Board>>,
) -> &'a HashSet<Rc<Board>> {
    while !board_stack.is_empty() {
        match board_stack.pop_back() {
            None => panic!("Board stack is empty!"),
            Some((board_rc, pieces)) => {
                let board = &*board_rc;
                for row in 1..=board.rows {
                    for col in 1..=board.cols {
                        let new_piece = Piece {
                            row,
                            col,
                            piece: pieces[0],
                        };
                        if board.is_safe(new_piece) {
                            let new_board = board_rc.place(new_piece);
                            let new_board_rc = Rc::new(new_board);
                            if pieces.len() != 1 {
                                if tested_configurations.insert(Rc::clone(&new_board_rc)) {
                                    let tail = &pieces[1..pieces.len()];
                                    board_stack.push_front((Rc::clone(&new_board_rc), tail));
                                }
                            } else {
                                solutions.insert(new_board_rc);
                            }
                        }
                    }
                }
            },
        }
    }
    solutions
}
