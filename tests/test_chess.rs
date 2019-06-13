use chess::*;
use std::{collections::BTreeSet, collections::HashSet, iter::FromIterator, vec::Vec};

#[test]
fn test_king_movements() {
    let piece_king = Piece {
        row: 5,
        col: 5,
        piece: ChessPiece::King,
    };
    let other_piece_1 = Piece {
        row: 5,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_1 = piece_king.attacks(other_piece_1);
    assert_eq!(attack_1, true);
    let other_piece_2 = Piece {
        row: 6,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_2 = piece_king.attacks(other_piece_2);
    assert_eq!(attack_2, true);
    let other_piece_3 = Piece {
        row: 5,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_3 = piece_king.attacks(other_piece_3);
    assert_eq!(attack_3, true);
    let other_piece_4 = Piece {
        row: 4,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_4 = piece_king.attacks(other_piece_4);
    assert_eq!(attack_4, true);
    let other_piece_5 = Piece {
        row: 4,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_5 = piece_king.attacks(other_piece_5);
    assert_eq!(attack_5, true);
    let other_piece_6 = Piece {
        row: 4,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_6 = piece_king.attacks(other_piece_6);
    assert_eq!(attack_6, true);
    let other_piece_7 = Piece {
        row: 6,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_7 = piece_king.attacks(other_piece_7);
    assert_eq!(attack_7, true);
    let other_piece_8 = Piece {
        row: 8,
        col: 8,
        piece: ChessPiece::King,
    };
    let attack_8 = piece_king.attacks(other_piece_8);
    assert_eq!(attack_8, false);
}
#[test]
fn test_bishop_movements() {
    let piece_bishop = Piece {
        row: 5,
        col: 5,
        piece: ChessPiece::Bishop,
    };
    let other_piece_1 = Piece {
        row: 7,
        col: 7,
        piece: ChessPiece::Bishop,
    };
    let attack_1 = piece_bishop.attacks(other_piece_1);
    assert_eq!(attack_1, true);
    let other_piece_2 = Piece {
        row: 6,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_2 = piece_bishop.attacks(other_piece_2);
    assert_eq!(attack_2, true);
    let other_piece_3 = Piece {
        row: 4,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_3 = piece_bishop.attacks(other_piece_3);
    assert_eq!(attack_3, true);
    let other_piece_4 = Piece {
        row: 3,
        col: 7,
        piece: ChessPiece::King,
    };
    let attack_4 = piece_bishop.attacks(other_piece_4);
    assert_eq!(attack_4, true);
    let other_piece_5 = Piece {
        row: 7,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_5 = piece_bishop.attacks(other_piece_5);
    assert_eq!(attack_5, true);
    let other_piece_6 = Piece {
        row: 6,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_6 = piece_bishop.attacks(other_piece_6);
    assert_eq!(attack_6, true);
    let other_piece_7 = Piece {
        row: 4,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_7 = piece_bishop.attacks(other_piece_7);
    assert_eq!(attack_7, true);

    let other_piece_8 = Piece {
        row: 3,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_8 = piece_bishop.attacks(other_piece_8);
    assert_eq!(attack_8, true);
    let other_piece_9 = Piece {
        row: 8,
        col: 1,
        piece: ChessPiece::King,
    };
    let attack_9 = piece_bishop.attacks(other_piece_9);
    assert_eq!(attack_9, false);
}
#[test]
fn test_knight_movements() {
    let piece_knight = Piece {
        row: 5,
        col: 5,
        piece: ChessPiece::Knight,
    };

    let other_piece_1 = Piece {
        row: 7,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_1 = piece_knight.attacks(other_piece_1);
    assert_eq!(attack_1, true);
    let other_piece_2 = Piece {
        row: 6,
        col: 7,
        piece: ChessPiece::King,
    };
    let attack_2 = piece_knight.attacks(other_piece_2);
    assert_eq!(attack_2, true);
    let other_piece_3 = Piece {
        row: 4,
        col: 7,
        piece: ChessPiece::King,
    };
    let attack_3 = piece_knight.attacks(other_piece_3);
    assert_eq!(attack_3, true);
    let other_piece_4 = Piece {
        row: 3,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_4 = piece_knight.attacks(other_piece_4);
    assert_eq!(attack_4, true);
    let other_piece_5 = Piece {
        row: 3,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_5 = piece_knight.attacks(other_piece_5);
    assert_eq!(attack_5, true);
    let other_piece_6 = Piece {
        row: 4,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_6 = piece_knight.attacks(other_piece_6);
    assert_eq!(attack_6, true);
    let other_piece_7 = Piece {
        row: 6,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_7 = piece_knight.attacks(other_piece_7);
    assert_eq!(attack_7, true);
    let other_piece_9 = Piece {
        row: 8,
        col: 1,
        piece: ChessPiece::King,
    };
    let attack_9 = piece_knight.attacks(other_piece_9);
    assert_eq!(attack_9, false);
}
#[test]
fn test_rook_movements() {
    let piece_rook = Piece {
        row: 5,
        col: 5,
        piece: ChessPiece::Rook,
    };

    let other_piece_1 = Piece {
        row: 6,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_1 = piece_rook.attacks(other_piece_1);
    assert_eq!(attack_1, true);
    let other_piece_2 = Piece {
        row: 7,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_2 = piece_rook.attacks(other_piece_2);
    assert_eq!(attack_2, true);
    let other_piece_3 = Piece {
        row: 4,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_3 = piece_rook.attacks(other_piece_3);
    assert_eq!(attack_3, true);
    let other_piece_4 = Piece {
        row: 3,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_4 = piece_rook.attacks(other_piece_4);
    assert_eq!(attack_4, true);
    let other_piece_5 = Piece {
        row: 5,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_5 = piece_rook.attacks(other_piece_5);
    assert_eq!(attack_5, true);
    let other_piece_6 = Piece {
        row: 5,
        col: 7,
        piece: ChessPiece::King,
    };
    let attack_6 = piece_rook.attacks(other_piece_6);
    assert_eq!(attack_6, true);
    let other_piece_7 = Piece {
        row: 5,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_7 = piece_rook.attacks(other_piece_7);
    assert_eq!(attack_7, true);
    let other_piece_9 = Piece {
        row: 5,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_9 = piece_rook.attacks(other_piece_9);
    assert_eq!(attack_9, true);
    let other_piece_10 = Piece {
        row: 7,
        col: 2,
        piece: ChessPiece::King,
    };
    let attack_10 = piece_rook.attacks(other_piece_10);
    assert_eq!(attack_10, false);
}
#[test]
fn test_queen_movements() {
    let piece_queen = Piece {
        row: 5,
        col: 5,
        piece: ChessPiece::Queen,
    };

    let other_piece_1 = Piece {
        row: 7,
        col: 7,
        piece: ChessPiece::King,
    };
    let attack_1 = piece_queen.attacks(other_piece_1);
    assert_eq!(attack_1, true);
    let other_piece_2 = Piece {
        row: 6,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_2 = piece_queen.attacks(other_piece_2);
    assert_eq!(attack_2, true);
    let other_piece_3 = Piece {
        row: 4,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_3 = piece_queen.attacks(other_piece_3);
    assert_eq!(attack_3, true);
    let other_piece_4 = Piece {
        row: 3,
        col: 7,
        piece: ChessPiece::King,
    };
    let attack_4 = piece_queen.attacks(other_piece_4);
    assert_eq!(attack_4, true);
    let other_piece_5 = Piece {
        row: 7,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_5 = piece_queen.attacks(other_piece_5);
    assert_eq!(attack_5, true);
    let other_piece_6 = Piece {
        row: 6,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_6 = piece_queen.attacks(other_piece_6);
    assert_eq!(attack_6, true);
    let other_piece_7 = Piece {
        row: 4,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_7 = piece_queen.attacks(other_piece_7);
    assert_eq!(attack_7, true);
    let other_piece_9 = Piece {
        row: 3,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_9 = piece_queen.attacks(other_piece_9);
    assert_eq!(attack_9, true);
    let other_piece_10 = Piece {
        row: 6,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_10 = piece_queen.attacks(other_piece_10);
    assert_eq!(attack_10, true);
    let other_piece_11 = Piece {
        row: 7,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_11 = piece_queen.attacks(other_piece_11);
    assert_eq!(attack_11, true);
    let other_piece_12 = Piece {
        row: 4,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_12 = piece_queen.attacks(other_piece_12);
    assert_eq!(attack_12, true);
    let other_piece_13 = Piece {
        row: 3,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_13 = piece_queen.attacks(other_piece_13);
    assert_eq!(attack_13, true);
    let other_piece_14 = Piece {
        row: 5,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_14 = piece_queen.attacks(other_piece_14);
    assert_eq!(attack_14, true);
    let other_piece_15 = Piece {
        row: 5,
        col: 7,
        piece: ChessPiece::King,
    };
    let attack_15 = piece_queen.attacks(other_piece_15);
    assert_eq!(attack_15, true);
    let other_piece_16 = Piece {
        row: 5,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_16 = piece_queen.attacks(other_piece_16);
    assert_eq!(attack_16, true);
    let other_piece_17 = Piece {
        row: 5,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_17 = piece_queen.attacks(other_piece_17);
    assert_eq!(attack_17, true);
    let other_piece_18 = Piece {
        row: 3,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_18 = piece_queen.attacks(other_piece_18);
    assert_eq!(attack_18, false);
}

#[test]
fn test_contains_board() {
    let piece_1 = Piece {
        row: 5,
        col: 6,
        piece: ChessPiece::King,
    };
    let piece_2 = Piece {
        row: 3,
        col: 4,
        piece: ChessPiece::King,
    };
    let pieces_1 = vec![piece_1, piece_2];
    let pieces_2 = vec![piece_2, piece_1];
    let board_a = Board {
        m: 3,
        n: 3,
        used_pieces: BTreeSet::from_iter(pieces_1.iter().cloned()),
    };
    let board_b = Board {
        m: 3,
        n: 3,
        used_pieces: BTreeSet::from_iter(pieces_2.iter().cloned()),
    };
    let mut hash_set = HashSet::new();
    hash_set.insert(board_a);
    assert!(hash_set.contains(&board_b));
}

#[test]
fn test_same_board() {
    let piece_1 = Piece {
        row: 5,
        col: 6,
        piece: ChessPiece::King,
    };
    let piece_2 = Piece {
        row: 3,
        col: 4,
        piece: ChessPiece::King,
    };
    let pieces_1 = vec![piece_1, piece_2];
    let pieces_2 = vec![piece_2, piece_1];
    let board_a = Board {
        m: 3,
        n: 3,
        used_pieces: BTreeSet::from_iter(pieces_1.iter().cloned()),
    };
    let board_b = Board {
        m: 3,
        n: 3,
        used_pieces: BTreeSet::from_iter(pieces_2.iter().cloned()),
    };
    assert_eq!(board_a, board_b);
}

#[test]
fn test_3x3_board_2K_1R() {
    let board = Board {
        m: 3,
        n: 3,
        used_pieces: BTreeSet::new(),
    };
    let pieces = vec![ChessPiece::King, ChessPiece::King, ChessPiece::Rook];
    let mut solutions: HashSet<Board> = HashSet::new();
    solution(board, pieces, &mut solutions, &mut HashSet::new());
    assert_eq!(4, solutions.len());
}

#[test]
fn test_board_4x4_2K_2Q_0B_0R_0N() {
    let board = Board {
        m: 4,
        n: 4,
        used_pieces: BTreeSet::new(),
    };
    let pieces = vec![
        ChessPiece::King,
        ChessPiece::King,
        ChessPiece::Queen,
        ChessPiece::Queen,
    ];
    let mut solutions: HashSet<Board> = HashSet::new();
    solution(board, pieces, &mut solutions, &mut HashSet::new());
    assert_eq!(20, solutions.len());
}

#[test]
fn test_board_4x4_0K_1Q_0B_2R_0N() {
    let board = Board {
        m: 4,
        n: 4,
        used_pieces: BTreeSet::new(),
    };
    let pieces = vec![ChessPiece::Queen, ChessPiece::Rook, ChessPiece::Rook];
    let mut solutions: HashSet<Board> = HashSet::new();
    solution(board, pieces, &mut solutions, &mut HashSet::new());
    assert_eq!(116, solutions.len());
}

#[test]
fn test_board_4x4_0K_2Q_1B_0R_0N() {
    let board = Board {
        m: 4,
        n: 4,
        used_pieces: BTreeSet::new(),
    };
    let pieces = vec![ChessPiece::Queen, ChessPiece::Queen, ChessPiece::Bishop];
    let mut solutions: HashSet<Board> = HashSet::new();
    solution(board, pieces, &mut solutions, &mut HashSet::new());
    assert_eq!(72, solutions.len());
}

#[test]
fn test_board_4x4_1Q_2R() {
    let board = Board {
        m: 4,
        n: 4,
        used_pieces: BTreeSet::new(),
    };
    let pieces = vec![ChessPiece::Queen, ChessPiece::Rook, ChessPiece::Rook];
    let mut solutions: HashSet<Board> = HashSet::new();
    solution(board, pieces, &mut solutions, &mut HashSet::new());
    assert_eq!(116, solutions.len());
}

#[test]
fn test_board_5x5_2K_2Q() {
    let board = Board {
        m: 5,
        n: 5,
        used_pieces: BTreeSet::new(),
    };
    let pieces = vec![
        ChessPiece::King,
        ChessPiece::King,
        ChessPiece::Queen,
        ChessPiece::Queen,
    ];
    let mut solutions: HashSet<Board> = HashSet::new();
    solution(board, pieces, &mut solutions, &mut HashSet::new());
    assert_eq!(816, solutions.len());
}

#[test]
fn test_board_5x5_1Q_2B() {
    let board = Board {
        m: 5,
        n: 5,
        used_pieces: BTreeSet::new(),
    };
    let pieces = vec![ChessPiece::Queen, ChessPiece::Bishop, ChessPiece::Bishop];
    let mut solutions: HashSet<Board> = HashSet::new();
    solution(board, pieces, &mut solutions, &mut HashSet::new());
    assert_eq!(1152, solutions.len());
}

#[test]
fn test_board_5x5_1Q_1B_1R() {
    let board = Board {
        m: 5,
        n: 5,
        used_pieces: BTreeSet::new(),
    };
    let pieces = vec![ChessPiece::Queen, ChessPiece::Bishop, ChessPiece::Rook];
    let mut solutions: HashSet<Board> = HashSet::new();
    solution(board, pieces, &mut solutions, &mut HashSet::new());
    assert_eq!(1224, solutions.len());
}

#[test]
fn test_board_5x5_2K_2B() {
    let board = Board {
        m: 5,
        n: 5,
        used_pieces: BTreeSet::new(),
    };
    let pieces = vec![
        ChessPiece::King,
        ChessPiece::King,
        ChessPiece::Bishop,
        ChessPiece::Bishop,
    ];
    let mut solutions: HashSet<Board> = HashSet::new();
    solution(board, pieces, &mut solutions, &mut HashSet::new());
    assert_eq!(7596, solutions.len());
}

#[test]
fn test_board_8x8_8Q() {
    let board = Board {
        m: 8,
        n: 8,
        used_pieces: BTreeSet::new(),
    };
    let pieces = vec![
        ChessPiece::Queen,
        ChessPiece::Queen,
        ChessPiece::Queen,
        ChessPiece::Queen,
        ChessPiece::Queen,
        ChessPiece::Queen,
        ChessPiece::Queen,
        ChessPiece::Queen,
    ];
    let mut solutions: HashSet<Board> = HashSet::new();
    solution(board, pieces, &mut solutions, &mut HashSet::new());
    assert_eq!(92, solutions.len());
}
