mod pawn;

use crate::chessboard::{
    BBISHOP, BKING, BKNIGHT, BPAWN, BQUEEN, BROOK, ChessBoard, EMPTY, Move, Players, WBISHOP,
    WKING, WKNIGHT, WPAWN, WQUEEN, WROOK,
};
use crate::moves::{
    ALL_DIRECTION_MOVES, ANTI_DIAGONAL_MOVES, BPAWN_ATTACK_MOVES, DIAGONAL_MOVES, LegalMove,
    MoveType, RatedMove, WPAWN_ATTACK_MOVES,
};

pub trait ChessEngine {
    fn legal_moves(&self) -> Vec<LegalMove>;

    fn perft(&self) -> Vec<(String, u32)>;
}

impl ChessEngine for ChessBoard {
    fn legal_moves(&self) -> Vec<LegalMove> {
        let legal_moves: Vec<LegalMove> = Vec::with_capacity(20);

        for square in self.board {
            if !(is_owned_piece(square, self.side_to_move)) {
                continue;
            }

            match square {
                WKING | BKING => {}

                WQUEEN | BQUEEN => {}

                WROOK | BROOK => {}

                WBISHOP | BBISHOP => {}

                WKNIGHT | BKNIGHT => {}

                WPAWN => {}

                BPAWN => {}

                EMPTY => {
                    continue;
                }

                _ => unreachable!("All cases have been covered!"),
            }
        }

        legal_moves
    }

    fn perft(&self) -> Vec<(String, u32)> {
        todo!()
    }
}

fn is_owned_piece(piece: i8, current_side: Players) -> bool {
    if current_side == Players::White {
        piece.is_positive()
    } else {
        piece.is_negative()
    }
}

fn get_multi_step_legal_moves(chessboard: &ChessBoard, move_data: &[Move]) -> Vec<LegalMove> {
    let legal_moves: Vec<LegalMove> = Vec::with_capacity(10);

    legal_moves
}

fn single_step_get_legal_moves(chessboard: &ChessBoard, move_data: &[Move]) -> Vec<LegalMove> {
    let legal_moves: Vec<LegalMove> = Vec::with_capacity(4);

    legal_moves
}

fn king_is_checked(board: &[i8; 64], king_position: usize) -> bool {
    /* Check per individual pieces, i.e. Diagonal moves: check only Queen, Bishop.. Anti-diagonal moves: check only Queen, Rook.. Pawn attacks */

    king_is_attacked_by_pawns(board, king_position)
        || king_is_attacked_on_diagonals(board, king_position)
        || king_is_attacked_on_anti_diagonals(board, king_position)
}

pub fn king_is_attacked_by_pawns(board: &[i8; 64], king_position: usize) -> bool {
    let (attack_moves, pawn) = if board[king_position].is_positive() {
        (WPAWN_ATTACK_MOVES, BPAWN)
    } else {
        (BPAWN_ATTACK_MOVES, WPAWN)
    };

    for attack_move in attack_moves {
        if let Some(pos) = attack_move.get_new_position(king_position) {
            if pawn == board[pos] {
                return true;
            }
        }
    }

    false
}

pub fn king_is_attacked_on_diagonals(board: &[i8; 64], king_position: usize) -> bool {
    /* No need to check for anything other than queen and bishops of opposing colors */
    let moves = DIAGONAL_MOVES;

    let danger_pieces = if board[king_position].is_positive() {
        [BQUEEN, BBISHOP]
    } else {
        [WQUEEN, WBISHOP]
    };

    for attack_move in moves {
        let mut pos_opt = attack_move.get_new_position(king_position);

        while pos_opt.is_some() {
            let new_pos = pos_opt.expect("prechecked for some variant");
            pos_opt = attack_move.get_new_position(new_pos);

            let square = board[new_pos];

            if square == EMPTY {
                continue;
            }

            if danger_pieces.contains(&square) {
                return true;
            } else {
                break;
            }
        }
    }

    false
}

pub fn king_is_attacked_on_anti_diagonals(board: &[i8; 64], king_position: usize) -> bool {
    /* No need to check for anything other than queen and bishops of opposing colors */
    let moves = ANTI_DIAGONAL_MOVES;

    let danger_pieces = if board[king_position].is_positive() {
        [BQUEEN, BROOK]
    } else {
        [WQUEEN, WROOK]
    };

    for attack_move in moves {
        let mut pos_opt = attack_move.get_new_position(king_position);

        while pos_opt.is_some() {
            let new_pos = pos_opt.expect("prechecked for some variant");
            pos_opt = attack_move.get_new_position(new_pos);

            let square = board[new_pos];

            if square == EMPTY {
                continue;
            }

            if danger_pieces.contains(&square) {
                return true;
            } else {
                break;
            }
        }
    }

    false
}
