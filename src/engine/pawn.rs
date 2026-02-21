use crate::{
    chessboard::{
        BBISHOP, BKNIGHT, BQUEEN, BROOK, ChessBoard, EMPTY, Move, Players, WBISHOP, WKNIGHT,
        WQUEEN, WROOK,
    },
    engine::pawn,
    moves::{
        BPAWN_ATTACK_MOVES, BPAWN_MOVE, BPAWN_START_MOVES, LegalMove, MoveType, WPAWN_ATTACK_MOVES,
        WPAWN_MOVE, WPAWN_START_MOVES,
    },
};

use super::{
    MoveStatus, can_move_to_position, check_promotion_and_generate_moves, is_en_passant_capture,
};

pub fn get_pawn_moves(position: usize, chessboard: &ChessBoard) -> Vec<LegalMove> {
    let mut moves = Vec::with_capacity(4);

    let promotions = if Players::White == chessboard.side_to_move {
        [WQUEEN, WROOK, WBISHOP, WKNIGHT]
    } else {
        [BQUEEN, BROOK, BBISHOP, BKNIGHT]
    };

    /*This is not correct for the black pawn, as the division rule does not apply correctly*/

    let (pawn_moves, attack_moves): (&[Move], [Move; 2]) =
        if chessboard.side_to_move == Players::White {
            if position > 7 && position < 16 {
                (&WPAWN_START_MOVES, WPAWN_ATTACK_MOVES)
            } else {
                (&[WPAWN_MOVE], WPAWN_ATTACK_MOVES)
            }
        } else if position > 47 && position < 56 {
            (&BPAWN_START_MOVES, BPAWN_ATTACK_MOVES)
        } else {
            (&[BPAWN_MOVE], BPAWN_ATTACK_MOVES)
        };

    for pawn_move in pawn_moves {
        let new_position = match pawn_move.get_new_position(position) {
            Some(pos) => pos,
            None => {
                break;
            }
        };

        let is_promotion_move = new_position / 8 == 7 || new_position / 8 == 0;

        match can_move_to_position(chessboard.side_to_move, chessboard.board[new_position]) {
            MoveStatus::NoMove | MoveStatus::CaptureMove => {
                break;
            }

            MoveStatus::Move => {
                if is_promotion_move {
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
                    } else if position - 8 == new_position {
                        MoveType::PawnMove {
                            promotion_move: None,
                        }
                    } else {
                        MoveType::PawnDoubleMove
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

    for attack_move in attack_moves {
        if let Some(new_position) = attack_move.get_new_position(position) {
            let can_capture = if chessboard.side_to_move == Players::White {
                |x: i8| x.is_negative()
            } else {
                |x: i8| x.is_positive()
            };

            if can_capture(chessboard.board[new_position])
                && chessboard.board[new_position] != EMPTY
            {
                if is_en_passant_capture(chessboard, new_position) {
                    let target_square = chessboard.en_passant_target_square.expect("checked above");
                    moves.push(LegalMove {
                        from: position,
                        to: new_position,
                        move_type: MoveType::Enpassant { target_square },
                        is_capture: true,
                    });
                } else {
                    moves.append(&mut check_promotion_and_generate_moves(
                        LegalMove {
                            from: position,
                            to: new_position,
                            move_type: MoveType::PawnMove {
                                promotion_move: None,
                            },
                            is_capture: true,
                        },
                        &promotions,
                    ));
                }
            }
        }
    }
    moves
}

// A simple move is a move that moves one rank, i.e. up or down.
fn get_simple_move(position: usize, chessboard: &ChessBoard) -> Option<LegalMove> {}

// Double moves are only possible if a pawn has not been moved. Moves up/down 2 ranks.
fn get_double_move(position: usize, chessboard: &ChessBoard) -> Option<LegalMove> {}

// A simple capture is a pawn moving diagonally one rank and one file onto an enemy piece, en passant not included.
fn get_simple_capture(position: usize, chessboard: &ChessBoard) -> Option<LegalMove> {}

// Self explanatory function.
fn get_en_passant_capture(position: usize, chessboard: &ChessBoard) -> Option<LegalMove> {}

// A promotion move is both a move, but also a capture move that ends in a promotion.
fn get_promotion(
    position: usize,
    chessboard: &ChessBoard,
    promotion_pieces: &[i8],
) -> Option<Vec<LegalMove>> {
}

fn get_simple_promotion(
    position: usize,
    chessboard: &ChessBoard,
    promotion_pieces: &[i8],
) -> Option<Vec<LegalMove>> {
    if let Some(pawn_move) = get_simple_move(position, chessboard) {
        let mut promotions: Vec<LegalMove> = Vec::with_capacity(4);

        for piece in promotion_pieces {
            let mut promotion_move = pawn_move;

            promotion_move.move_type = MoveType::PawnMove {
                promotion_move: Some(*piece),
            };
            promotions.push(promotion_move);
        }
        return Some(promotions);
    }
    None
}

fn get_simple_capture_promotion(
    position: usize,
    chessboard: &ChessBoard,
    promotion_pieces: &[i8],
) -> Option<Vec<LegalMove>> {
    if let Some(capture_move) = get_simple_capture(position, chessboard) {
        let mut promotions: Vec<LegalMove> = Vec::with_capacity(4);

        for piece in promotion_pieces {
            let mut promotion_capture = capture_move;

            promotion_capture.move_type = MoveType::PawnMove {
                promotion_move: Some(*piece),
            };
            promotions.push(promotion_capture);
        }
        return Some(promotions);
    }
    None
}

pub fn get_pawn_moves_v2(position: usize, chessboard: &ChessBoard) -> Vec<LegalMove> {
    let mut moves = Vec::new();

    let promotions = if Players::White == chessboard.side_to_move {
        [WQUEEN, WROOK, WBISHOP, WKNIGHT]
    } else {
        [BQUEEN, BROOK, BBISHOP, BKNIGHT]
    };

    // current rank, zero indexed
    let current_rank = position / 8;

    let (double_pawn_rank, promotion_rank, en_passant_rank) =
        if Players::White == chessboard.side_to_move {
            (1, 6, 4)
        } else {
            (6, 1, 3)
        };

    // Checking different cases

    if current_rank == promotion_rank {
        if let Some(mut promotions) = get_promotion(position, chessboard, &promotions) {
            moves.append(&mut promotions);
        }
    } else {
        if let Some(simple_move) = get_simple_capture(position, chessboard) {
            moves.push(simple_move)
        }

        if let Some(simple_attack) = get_simple_capture(position, chessboard) {
            moves.push(simple_attack);
        }
    }

    if double_pawn_rank == current_rank {
        if let Some(double_pawn_move) = get_double_move(position, chessboard) {
            moves.push(double_pawn_move);
        }
    }

    if current_rank == en_passant_rank && chessboard.en_passant_target_square.is_some() {
        if let Some(en_passant_move) = get_en_passant_capture(position, chessboard) {
            moves.push(en_passant_move);
        }
    }

    moves
}
