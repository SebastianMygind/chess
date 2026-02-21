use crate::{
    chessboard::{
        BBISHOP, BKNIGHT, BQUEEN, BROOK, ChessBoard, EMPTY, Move, Players, WBISHOP, WKNIGHT,
        WQUEEN, WROOK,
    },
    engine::utils::fold_pawn_moves,
    moves::{LegalMove, MoveType},
};

const WPAWN_MOVE: Move = Move { dx: 0, dy: 1 };
const WPAWN_DOUBLE_MOVE: Move = Move { dx: 0, dy: 2 };
pub const WPAWN_ATTACK_MOVES: [Move; 2] = [Move { dx: 1, dy: 1 }, Move { dx: -1, dy: 1 }];

const BPAWN_MOVE: Move = Move { dx: 0, dy: -1 };
const BPAWN_DOUBLE_MOVE: Move = Move { dx: 0, dy: -2 };
pub const BPAWN_ATTACK_MOVES: [Move; 2] = [Move { dx: 1, dy: -1 }, Move { dx: -1, dy: -1 }];

const WPAWN_MOVES: [Move; 4] = fold_pawn_moves(WPAWN_MOVE, WPAWN_DOUBLE_MOVE, WPAWN_ATTACK_MOVES);
const BPAWN_MOVES: [Move; 4] = fold_pawn_moves(BPAWN_MOVE, BPAWN_DOUBLE_MOVE, BPAWN_ATTACK_MOVES);

const SINGLE_MOVE_INDEX: usize = 0;
const DOUBLE_MOVE_INDEX: usize = 1;
const ATTACK_START_INDEX: usize = 2;
// predicate functions to know if a pawn can capture a square.
fn is_white_capturable(square: i8) -> bool {
    square.is_negative()
}

fn is_black_capturable(square: i8) -> bool {
    square.is_positive()
}

// A simple move is a move that moves one rank, i.e. up or down.
fn get_simple_move(
    position: usize,
    chessboard: &ChessBoard,
    pawn_moves: &[Move],
) -> Option<LegalMove> {
    let new_position: usize;
    if let Some(position_opt) = pawn_moves[SINGLE_MOVE_INDEX].get_new_position(position) {
        new_position = position_opt;
    } else {
        return None;
    }

    if chessboard.board[new_position] != EMPTY {
        return None;
    }

    let pawn_move = LegalMove {
        from: position,
        to: new_position,
        move_type: MoveType::PawnMove {
            promotion_move: None,
        },
        is_capture: false,
    };

    Some(pawn_move)
}

// Double moves are only possible if a pawn has not been moved. Moves up/down 2 ranks.
fn get_double_move(
    position: usize,
    chessboard: &ChessBoard,
    pawn_moves: &[Move],
) -> Option<LegalMove> {
    let mut new_position: usize;

    if let Some(pos_opt) = pawn_moves[SINGLE_MOVE_INDEX].get_new_position(position) {
        new_position = pos_opt;
    } else {
        return None;
    }

    if chessboard.board[new_position] != EMPTY {
        return None;
    }

    if let Some(pos_opt) = pawn_moves[DOUBLE_MOVE_INDEX].get_new_position(position) {
        new_position = pos_opt;
    } else {
        return None;
    }

    if chessboard.board[new_position] != EMPTY {
        return None;
    }

    let double_move = LegalMove {
        from: position,
        to: new_position,
        move_type: MoveType::PawnDoubleMove,
        is_capture: false,
    };

    Some(double_move)
}

// A simple capture is a pawn moving diagonally one rank and one file onto an enemy piece, en passant not included.
fn get_simple_capture(
    position: usize,
    chessboard: &ChessBoard,
    pawn_moves: &[Move],
    can_capture: fn(i8) -> bool,
) -> Option<Vec<LegalMove>> {
    let mut captures: Vec<LegalMove> = Vec::with_capacity(2);

    for attack in pawn_moves[ATTACK_START_INDEX..].iter() {
        let new_position: usize;

        if let Some(position_opt) = attack.get_new_position(position) {
            new_position = position_opt;
        } else {
            continue;
        }

        let square = chessboard.board[new_position];

        if !can_capture(square) {
            continue;
        }

        let capture_move = LegalMove {
            from: position,
            to: new_position,
            move_type: MoveType::PawnMove {
                promotion_move: None,
            },
            is_capture: true,
        };
        captures.push(capture_move);
    }

    if captures.is_empty() {
        None
    } else {
        Some(captures)
    }
}

// Self explanatory function.
fn get_en_passant_capture(
    position: usize,
    chessboard: &ChessBoard,
    pawn_moves: &[Move],
) -> Option<LegalMove> {
    let target_pawn_square = chessboard
        .en_passant_target_square
        .expect("Checked before calling function.");

    //the position a capturing pawn would end up at after taking the en passant pawn.
    let en_passant_position = if chessboard.side_to_move == Players::White {
        target_pawn_square + 8
    } else {
        target_pawn_square - 8
    };

    let mut can_capture = false;

    for direction in pawn_moves[ATTACK_START_INDEX..].iter() {
        if let Some(new_position) = direction.get_new_position(position)
            && new_position == en_passant_position
            && chessboard.board[new_position] == EMPTY
        {
            can_capture = true;
            break;
        }
    }

    if !can_capture {
        return None;
    }

    let en_passant_move = LegalMove {
        from: position,
        to: en_passant_position,
        move_type: MoveType::Enpassant {
            target_square: target_pawn_square,
        },
        is_capture: true,
    };

    Some(en_passant_move)
}

fn get_simple_promotion(
    position: usize,
    chessboard: &ChessBoard,
    pawn_moves: &[Move],
    promotion_pieces: &[i8],
) -> Option<Vec<LegalMove>> {
    if let Some(pawn_move) = get_simple_move(position, chessboard, pawn_moves) {
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
    pawn_moves: &[Move],
    promotion_pieces: &[i8],
    can_capture: fn(i8) -> bool,
) -> Option<Vec<LegalMove>> {
    let mut promotion_moves: Vec<LegalMove> = Vec::with_capacity(4);

    let capture_moves = get_simple_capture(position, chessboard, pawn_moves, can_capture)?;

    for capture_move in capture_moves {
        for piece in promotion_pieces {
            let mut promotion_capture = capture_move;

            promotion_capture.move_type = MoveType::PawnMove {
                promotion_move: Some(*piece),
            };
            promotion_moves.push(promotion_capture);
        }
    }

    Some(promotion_moves)
}

pub fn get_pawn_moves(position: usize, chessboard: &ChessBoard) -> Vec<LegalMove> {
    let mut moves = Vec::new();

    // Variables dependent on which side is to move.
    let pawn_moves: &[Move];
    let promotions: &[i8];
    let double_pawn_rank: usize;
    let promotion_rank: usize;
    let en_passant_rank: usize;
    let can_capture: fn(i8) -> bool;

    // Setting variables dependent on which side is to move.
    if Players::White == chessboard.side_to_move {
        pawn_moves = &WPAWN_MOVES;
        promotions = &[WQUEEN, WROOK, WBISHOP, WKNIGHT];
        double_pawn_rank = 1;
        promotion_rank = 6;
        en_passant_rank = 4;
        can_capture = is_white_capturable;
    } else {
        pawn_moves = &BPAWN_MOVES;
        promotions = &[BQUEEN, BROOK, BBISHOP, BKNIGHT];
        double_pawn_rank = 6;
        promotion_rank = 1;
        en_passant_rank = 3;
        can_capture = is_black_capturable;
    }

    // current rank, zero indexed
    let current_rank = position / 8;

    // Checking different cases

    if current_rank == promotion_rank {
        if let Some(mut promotions) =
            get_simple_promotion(position, chessboard, pawn_moves, promotions)
        {
            moves.append(&mut promotions);
        }

        if let Some(mut promotions) =
            get_simple_capture_promotion(position, chessboard, pawn_moves, promotions, can_capture)
        {
            moves.append(&mut promotions);
        }
    } else {
        if let Some(simple_move) = get_simple_move(position, chessboard, pawn_moves) {
            moves.push(simple_move)
        }

        if let Some(mut simple_attack) =
            get_simple_capture(position, chessboard, pawn_moves, can_capture)
        {
            moves.append(&mut simple_attack);
        }
    }

    if double_pawn_rank == current_rank
        && let Some(double_pawn_move) = get_double_move(position, chessboard, pawn_moves)
    {
        moves.push(double_pawn_move);
    }

    if current_rank == en_passant_rank
        && chessboard.en_passant_target_square.is_some()
        && let Some(en_passant_move) = get_en_passant_capture(position, chessboard, pawn_moves)
    {
        moves.push(en_passant_move);
    }

    moves
}
