mod pawn;

use crate::chessboard::ChessBoard;
use crate::moves::{LegalMove, MoveType, RatedMove};

pub trait ChessEngine {
    fn legal_moves(&self) -> Vec<LegalMove>;

    fn perft(&self) -> Vec<(String, u32)>;
}

impl ChessEngine for ChessBoard {
    fn legal_moves(&self) -> Vec<LegalMove> {
        todo!()
    }

    fn perft(&self) -> Vec<(String, u32)> {
        todo!()
    }
}
