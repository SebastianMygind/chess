use crate::{
    chessboard::ChessBoard,
    moves::{ANTI_DIAGONAL_MOVES, LegalMove, MoveType},
};

use super::get_multi_step_pseudo_legal_moves;

pub fn get_rook_moves(position: usize, chessboard: &ChessBoard) -> Vec<LegalMove> {
    get_multi_step_pseudo_legal_moves(
        chessboard,
        &ANTI_DIAGONAL_MOVES,
        position,
        MoveType::RookMove,
    )
}
