use crate::{
    chessboard::{ChessBoard, EMPTY, Players},
    moves::{ALL_DIRECTION_MOVES, LegalMove, MoveType},
};

use super::single_step_get_pseudo_legal_moves;

pub fn get_king_moves(position: usize, chessboard: &ChessBoard) -> Vec<LegalMove> {
    let mut moves = single_step_get_pseudo_legal_moves(
        chessboard,
        &ALL_DIRECTION_MOVES,
        position,
        MoveType::KingMove,
    );

    let (king_ability, queen_ability) = if chessboard.side_to_move == Players::White {
        (
            chessboard.castling_ability[0],
            chessboard.castling_ability[1],
        )
    } else {
        (
            chessboard.castling_ability[2],
            chessboard.castling_ability[3],
        )
    };
    if king_ability {
        if chessboard.board[position + 1] == EMPTY && chessboard.board[position + 2] == EMPTY {
            moves.push(LegalMove {
                from: position,
                to: position + 2,
                move_type: MoveType::CastleKingSide,
                is_capture: false,
            });
        }
    }

    if queen_ability {
        if chessboard.board[position - 1] == EMPTY
            && chessboard.board[position - 2] == EMPTY
            && chessboard.board[position - 3] == EMPTY
        {
            moves.push(LegalMove {
                from: position,
                to: position - 2,
                move_type: MoveType::CastleQueenSide,
                is_capture: false,
            });
        }
    }

    moves
}
