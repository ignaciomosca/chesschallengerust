use super::*;

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
    let attack_1 = piece_king.attacks(&other_piece_1);
    assert_eq!(attack_1, true);
    let other_piece_2 = Piece {
        row: 6,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_2 = piece_king.attacks(&other_piece_2);
    assert_eq!(attack_2, true);
    let other_piece_3 = Piece {
        row: 5,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_3 = piece_king.attacks(&other_piece_3);
    assert_eq!(attack_3, true);
    let other_piece_4 = Piece {
        row: 4,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_4 = piece_king.attacks(&other_piece_4);
    assert_eq!(attack_4, true);
    let other_piece_5 = Piece {
        row: 4,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_5 = piece_king.attacks(&other_piece_5);
    assert_eq!(attack_5, true);
    let other_piece_6 = Piece {
        row: 4,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_6 = piece_king.attacks(&other_piece_6);
    assert_eq!(attack_6, true);
    let other_piece_7 = Piece {
        row: 6,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_7 = piece_king.attacks(&other_piece_7);
    assert_eq!(attack_7, true);
    let other_piece_8 = Piece {
        row: 8,
        col: 8,
        piece: ChessPiece::King,
    };
    let attack_8 = piece_king.attacks(&other_piece_8);
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
    let attack_1 = piece_bishop.attacks(&other_piece_1);
    assert_eq!(attack_1, true);
    let other_piece_2 = Piece {
        row: 6,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_2 = piece_bishop.attacks(&other_piece_2);
    assert_eq!(attack_2, true);
    let other_piece_3 = Piece {
        row: 4,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_3 = piece_bishop.attacks(&other_piece_3);
    assert_eq!(attack_3, true);
    let other_piece_4 = Piece {
        row: 3,
        col: 7,
        piece: ChessPiece::King,
    };
    let attack_4 = piece_bishop.attacks(&other_piece_4);
    assert_eq!(attack_4, true);
    let other_piece_5 = Piece {
        row: 7,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_5 = piece_bishop.attacks(&other_piece_5);
    assert_eq!(attack_5, true);
    let other_piece_6 = Piece {
        row: 6,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_6 = piece_bishop.attacks(&other_piece_6);
    assert_eq!(attack_6, true);
    let other_piece_7 = Piece {
        row: 4,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_7 = piece_bishop.attacks(&other_piece_7);
    assert_eq!(attack_7, true);

    let other_piece_8 = Piece {
        row: 3,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_8 = piece_bishop.attacks(&other_piece_8);
    assert_eq!(attack_8, true);
    let other_piece_9 = Piece {
        row: 8,
        col: 1,
        piece: ChessPiece::King,
    };
    let attack_9 = piece_bishop.attacks(&other_piece_9);
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
    let attack_1 = piece_knight.attacks(&other_piece_1);
    assert_eq!(attack_1, true);
    let other_piece_2 = Piece {
        row: 6,
        col: 7,
        piece: ChessPiece::King,
    };
    let attack_2 = piece_knight.attacks(&other_piece_2);
    assert_eq!(attack_2, true);
    let other_piece_3 = Piece {
        row: 4,
        col: 7,
        piece: ChessPiece::King,
    };
    let attack_3 = piece_knight.attacks(&other_piece_3);
    assert_eq!(attack_3, true);
    let other_piece_4 = Piece {
        row: 3,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_4 = piece_knight.attacks(&other_piece_4);
    assert_eq!(attack_4, true);
    let other_piece_5 = Piece {
        row: 3,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_5 = piece_knight.attacks(&other_piece_5);
    assert_eq!(attack_5, true);
    let other_piece_6 = Piece {
        row: 4,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_6 = piece_knight.attacks(&other_piece_6);
    assert_eq!(attack_6, true);
    let other_piece_7 = Piece {
        row: 6,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_7 = piece_knight.attacks(&other_piece_7);
    assert_eq!(attack_7, true);
    let other_piece_9 = Piece {
        row: 8,
        col: 1,
        piece: ChessPiece::King,
    };
    let attack_9 = piece_knight.attacks(&other_piece_9);
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
    let attack_1 = piece_rook.attacks(&other_piece_1);
    assert_eq!(attack_1, true);
    let other_piece_2 = Piece {
        row: 7,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_2 = piece_rook.attacks(&other_piece_2);
    assert_eq!(attack_2, true);
    let other_piece_3 = Piece {
        row: 4,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_3 = piece_rook.attacks(&other_piece_3);
    assert_eq!(attack_3, true);
    let other_piece_4 = Piece {
        row: 3,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_4 = piece_rook.attacks(&other_piece_4);
    assert_eq!(attack_4, true);
    let other_piece_5 = Piece {
        row: 5,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_5 = piece_rook.attacks(&other_piece_5);
    assert_eq!(attack_5, true);
    let other_piece_6 = Piece {
        row: 5,
        col: 7,
        piece: ChessPiece::King,
    };
    let attack_6 = piece_rook.attacks(&other_piece_6);
    assert_eq!(attack_6, true);
    let other_piece_7 = Piece {
        row: 5,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_7 = piece_rook.attacks(&other_piece_7);
    assert_eq!(attack_7, true);
    let other_piece_9 = Piece {
        row: 5,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_9 = piece_rook.attacks(&other_piece_9);
    assert_eq!(attack_9, true);
    let other_piece_10 = Piece {
        row: 7,
        col: 2,
        piece: ChessPiece::King,
    };
    let attack_10 = piece_rook.attacks(&other_piece_10);
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
    let attack_1 = piece_queen.attacks(&other_piece_1);
    assert_eq!(attack_1, true);
    let other_piece_2 = Piece {
        row: 6,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_2 = piece_queen.attacks(&other_piece_2);
    assert_eq!(attack_2, true);
    let other_piece_3 = Piece {
        row: 4,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_3 = piece_queen.attacks(&other_piece_3);
    assert_eq!(attack_3, true);
    let other_piece_4 = Piece {
        row: 3,
        col: 7,
        piece: ChessPiece::King,
    };
    let attack_4 = piece_queen.attacks(&other_piece_4);
    assert_eq!(attack_4, true);
    let other_piece_5 = Piece {
        row: 7,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_5 = piece_queen.attacks(&other_piece_5);
    assert_eq!(attack_5, true);
    let other_piece_6 = Piece {
        row: 6,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_6 = piece_queen.attacks(&other_piece_6);
    assert_eq!(attack_6, true);
    let other_piece_7 = Piece {
        row: 4,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_7 = piece_queen.attacks(&other_piece_7);
    assert_eq!(attack_7, true);
    let other_piece_9 = Piece {
        row: 3,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_9 = piece_queen.attacks(&other_piece_9);
    assert_eq!(attack_9, true);
    let other_piece_10 = Piece {
        row: 6,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_10 = piece_queen.attacks(&other_piece_10);
    assert_eq!(attack_10, true);
    let other_piece_11 = Piece {
        row: 7,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_11 = piece_queen.attacks(&other_piece_11);
    assert_eq!(attack_11, true);
    let other_piece_12 = Piece {
        row: 4,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_12 = piece_queen.attacks(&other_piece_12);
    assert_eq!(attack_12, true);
    let other_piece_13 = Piece {
        row: 3,
        col: 5,
        piece: ChessPiece::King,
    };
    let attack_13 = piece_queen.attacks(&other_piece_13);
    assert_eq!(attack_13, true);
    let other_piece_14 = Piece {
        row: 5,
        col: 6,
        piece: ChessPiece::King,
    };
    let attack_14 = piece_queen.attacks(&other_piece_14);
    assert_eq!(attack_14, true);
    let other_piece_15 = Piece {
        row: 5,
        col: 7,
        piece: ChessPiece::King,
    };
    let attack_15 = piece_queen.attacks(&other_piece_15);
    assert_eq!(attack_15, true);
    let other_piece_16 = Piece {
        row: 5,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_16 = piece_queen.attacks(&other_piece_16);
    assert_eq!(attack_16, true);
    let other_piece_17 = Piece {
        row: 5,
        col: 3,
        piece: ChessPiece::King,
    };
    let attack_17 = piece_queen.attacks(&other_piece_17);
    assert_eq!(attack_17, true);
    let other_piece_18 = Piece {
        row: 3,
        col: 4,
        piece: ChessPiece::King,
    };
    let attack_18 = piece_queen.attacks(&other_piece_18);
    assert_eq!(attack_18, false);
}

#[test]
fn test_board() {
    let board = Board {
        m: 7,
        n: 7,
        used_pieces: HashSet::new(),
        number_of_pieces: 7,
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
    let solutions = solution(board, pieces, HashSet::new());
    assert_eq!(3063828, solutions.len());
}
