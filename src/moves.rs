use std::str::FromStr;

use crate::{
    chessboard::{BBISHOP, BKNIGHT, BQUEEN, BROOK, Move, WBISHOP, WKNIGHT, WQUEEN, WROOK},
    fen::parsing::parse_rank,
};

#[derive(Debug, Clone, Copy, PartialEq)]
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

pub const fn concat_const_arrays(arr_1: [Move; 4], arr_2: [Move; 4]) -> [Move; 8] {
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

fn file_to_char(file: usize) -> char {
    match file {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => unreachable!("The file of a piece should never be above the 7th or h file"),
    }
}

fn promotion_to_char(promotion: i8) -> char {
    match promotion {
        WQUEEN | BQUEEN => 'q',
        WROOK | BROOK => 'r',
        WBISHOP | BBISHOP => 'b',
        WKNIGHT | BKNIGHT => 'n',
        _ => unreachable!("not an allowed promotion"),
    }
}

/*
 * Output a legal move in UCI-compatible LAN format (Long Algebraic Notation)
 */
impl ToString for LegalMove {
    fn to_string(&self) -> String {
        let from_file = file_to_char(self.from % 8);
        let from_rank = (self.from / 8) + 1;

        let to_file = file_to_char(self.to % 8);
        let to_rank = (self.to / 8) + 1;

        let mut move_string = format!("{from_file}{from_rank}{to_file}{to_rank}").to_string();

        if let MoveType::PawnMove {
            promotion_move: Some(promotion),
        } = self.move_type
        {
            move_string.push(promotion_to_char(promotion));
        }
        move_string
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LegalMoveParseError;

/*
 * Parse a legal move from a UCI-compatible LAN string.
 */
impl FromStr for LegalMove {
    type Err = LegalMoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Self {
            from: 0,
            to: 0,
            move_type: MoveType::Normal,
            is_capture: false,
        });
    }
}
