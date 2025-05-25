use crate::{
    chessboard::ChessBoard,
    moves::{DIAGONAL_MOVES, LegalMove, MoveType},
};

use super::get_multi_step_pseudo_legal_moves;

pub fn get_bishop_moves(position: usize, chessboard: &ChessBoard) -> Vec<LegalMove> {
    get_multi_step_pseudo_legal_moves(chessboard, &DIAGONAL_MOVES, position, MoveType::Normal)
}
