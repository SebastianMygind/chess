use crate::{
    chessboard::{ChessBoard, EMPTY, Players},
    moves::{ALL_DIRECTION_MOVES, LegalMove, MoveType},
};

use super::get_multi_step_pseudo_legal_moves;

pub fn get_queen_moves(position: usize, chessboard: &ChessBoard) -> Vec<LegalMove> {
    get_multi_step_pseudo_legal_moves(chessboard, &ALL_DIRECTION_MOVES, position, MoveType::Normal)
}
