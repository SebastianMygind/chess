mod pawn;

use crate::chessboard::{
    BBISHOP, BKING, BKNIGHT, BPAWN, BQUEEN, BROOK, ChessBoard, EMPTY, Move, Players, WBISHOP,
    WKING, WKNIGHT, WPAWN, WQUEEN, WROOK,
};
use crate::moves::{ALL_DIRECTION_MOVES, LegalMove, MoveType, RatedMove};

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

fn king_is_checked(board: &[i8; 64], king_position: usize, attacking_side: Players) -> bool {
    /* Check per individual pieces, i.e. Diagonal moves: check only Queen, Bishop.. Anti-diagonal moves: check only Queen, Rook.. Pawn attacks */

    false
}
