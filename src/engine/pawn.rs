use crate::{
    chessboard::{
        BBISHOP, BKNIGHT, BQUEEN, BROOK, ChessBoard, Move, Players, WBISHOP, WKNIGHT, WQUEEN, WROOK,
    },
    moves::{
        BPAWN_ATTACK_MOVES, BPAWN_MOVE, BPAWN_START_MOVES, LegalMove, MoveType, WPAWN_ATTACK_MOVES,
        WPAWN_MOVE, WPAWN_START_MOVES,
    },
};

use super::{MoveStatus, can_move_to_position};

pub fn get_pawn_moves(position: usize, chessboard: &ChessBoard) -> Vec<LegalMove> {
    let mut moves = Vec::with_capacity(4);

    let (pawn_moves, attack_moves): (&[Move], [Move; 2]) = if position / 8 == 1 {
        if chessboard.side_to_move == Players::White {
            (&WPAWN_START_MOVES, WPAWN_ATTACK_MOVES)
        } else {
            (&BPAWN_START_MOVES, BPAWN_ATTACK_MOVES)
        }
    } else {
        if chessboard.side_to_move == Players::White {
            (&[WPAWN_MOVE], WPAWN_ATTACK_MOVES)
        } else {
            (&[BPAWN_MOVE], BPAWN_ATTACK_MOVES)
        }
    };

    for pawn_move in pawn_moves {
        let new_position = match pawn_move.get_new_position(position) {
            Some(pos) => pos,
            None => {
                break;
            }
        };

        let is_promotion_move = if new_position / 8 == 7 || new_position / 8 == 0 {
            true
        } else {
            false
        };

        match can_move_to_position(chessboard.side_to_move, chessboard.board[new_position]) {
            MoveStatus::NoMove | MoveStatus::CaptureMove => {
                break;
            }

            MoveStatus::Move => {
                if is_promotion_move {
                    let promotions = if Players::White == chessboard.side_to_move {
                        [WQUEEN, WROOK, WBISHOP, WKNIGHT]
                    } else {
                        [BQUEEN, BROOK, BBISHOP, BKNIGHT]
                    };

                    for promotion in promotions {
                        moves.push(LegalMove {
                            from: position,
                            to: new_position,
                            move_type: MoveType::PawnMove {
                                promotion_move: Some(promotion),
                            },
                            is_capture: false,
                        });
                    }
                } else {
                    let move_type = if Players::White == chessboard.side_to_move {
                        if position + 8 == new_position {
                            MoveType::PawnMove {
                                promotion_move: None,
                            }
                        } else {
                            MoveType::PawnDoubleMove
                        }
                    } else {
                        if position - 8 == new_position {
                            MoveType::PawnMove {
                                promotion_move: None,
                            }
                        } else {
                            MoveType::PawnDoubleMove
                        }
                    };

                    moves.push(LegalMove {
                        from: position,
                        to: new_position,
                        move_type,
                        is_capture: false,
                    });
                }
            }
        }
    }
    moves
}
