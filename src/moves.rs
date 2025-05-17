use crate::chessboard::{MAIL_BOX_64, MAIL_BOX_120, Move};

#[derive(Debug, Clone, Copy)]
pub enum MoveType {
    Normal,
    PawnMove { promotion_move: Option<i8> },
    PawnDoubleMove,
    Enpassant { target_square: usize }, /* The square at which the pawn to take exists at */
    KingMove,
    RookMove,
    CastleKingSide,
    CastleQueenSide,
}

#[derive(Debug, Clone, Copy)]
pub struct LegalMove {
    pub from: usize,
    pub to: usize,
    pub move_type: MoveType,
    pub is_capture: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct RatedMove {
    chess_move: LegalMove,
    rating: i32,
}

pub const WPAWN_START_MOVES: [Move; 2] = [Move { dx: 0, dy: 1 }, Move { dx: 0, dy: 2 }];
pub const WPAWN_MOVE: Move = Move { dx: 0, dy: 1 };
pub const WPAWN_ATTACK_MOVES: [Move; 2] = [Move { dx: 1, dy: 1 }, Move { dx: -1, dy: 1 }];

pub const BPAWN_START_MOVES: [Move; 2] = [Move { dx: 0, dy: -1 }, Move { dx: 0, dy: -2 }];
pub const BPAWN_MOVE: Move = Move { dx: 0, dy: -1 };
pub const BPAWN_ATTACK_MOVES: [Move; 2] = [Move { dx: 1, dy: -1 }, Move { dx: -1, dy: -1 }];

pub const KNIGHT_MOVES: [Move; 8] = [
    Move { dx: 1, dy: 2 },
    Move { dx: 1, dy: -2 },
    Move { dx: 2, dy: 1 },
    Move { dx: 2, dy: -1 },
    Move { dx: -1, dy: 2 },
    Move { dx: -1, dy: -2 },
    Move { dx: -2, dy: 1 },
    Move { dx: -2, dy: -1 },
];

pub const DIAGONAL_MOVES: [Move; 4] = [
    Move { dx: 1, dy: 1 },
    Move { dx: -1, dy: 1 },
    Move { dx: 1, dy: -1 },
    Move { dx: -1, dy: -1 },
];

pub const ANTI_DIAGONAL_MOVES: [Move; 4] = [
    Move { dx: 1, dy: 0 },
    Move { dx: -1, dy: 0 },
    Move { dx: 0, dy: 1 },
    Move { dx: 0, dy: -1 },
];

pub const ALL_DIRECTION_MOVES: [Move; 8] = concat_const_arrays(DIAGONAL_MOVES, ANTI_DIAGONAL_MOVES);

const fn concat_const_arrays(arr_1: [Move; 4], arr_2: [Move; 4]) -> [Move; 8] {
    let mut moves: [Move; 8] = [Move { dx: 0, dy: 0 }; 8];
    let mut index = 0;
    let mut i = 0;

    while i < 4 {
        moves[index] = arr_1[i];
        index += 1;
        i += 1;
    }
    i = 0;
    while i < 4 {
        moves[index] = arr_2[i];
        index += 1;
        i += 1;
    }

    moves
}
