use crate::{
    chessboard::ChessBoard,
    moves::{KNIGHT_MOVES, LegalMove, MoveType},
};

use super::single_step_get_pseudo_legal_moves;

pub fn get_knight_moves(position: usize, chessboard: &ChessBoard) -> Vec<LegalMove> {
    single_step_get_pseudo_legal_moves(chessboard, &KNIGHT_MOVES, position, MoveType::Normal)
}
