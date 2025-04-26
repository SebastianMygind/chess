use crate::fen::implementation::{
    parse_castling_ability, parse_epawn, parse_fen_part, parse_file, parse_position, parse_rank,
    parse_side_to_move, parse_string_to_num,
};
use crate::fen::{FEN_STARTING_POSITION, Fen, FenArguments, FenError, FenType};

use std::fmt;

pub const BOARD_HEIGHT: usize = 8;
pub const BOARD_WIDTH: usize = 8;

pub const WPAWN: i8 = 1;
pub const WKNIGHT: i8 = 2;
pub const WBISHOP: i8 = 3;
pub const WROOK: i8 = 4;
pub const WQUEEN: i8 = 5;
pub const WKING: i8 = 6;

pub const EMPTY: i8 = 0;

pub const BPAWN: i8 = -1;
pub const BKNIGHT: i8 = -2;
pub const BBISHOP: i8 = -3;
pub const BROOK: i8 = -4;
pub const BQUEEN: i8 = -5;
pub const BKING: i8 = -6;

/*  The mail box is an efficient way to check for bounds during chess moves, as you either get -1 or the position on the Chessboard.board.
    It can be more easily visualised with this board visualisation:
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1,  0,  1,  2,  3,  4,  5,  6,  7, -1,
    -1,  8,  9, 10, 11, 12, 13, 14, 15, -1,
    -1, 16, 17, 18, 19, 20, 21, 22, 23, -1,
    -1, 24, 25, 26, 27, 28, 29, 30, 31, -1,
    -1, 32, 33, 34, 35, 36, 37, 38, 39, -1,
    -1, 40, 41, 42, 43, 44, 45, 46, 47, -1,
    -1, 48, 49, 50, 51, 52, 53, 54, 55, -1,
    -1, 56, 57, 58, 59, 60, 61, 62, 63, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
*/
pub const MAIL_BOX_120: [i8; (BOARD_WIDTH + 2) * (BOARD_HEIGHT + 4)] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3,
    4, 5, 6, 7, -1, -1, 8, 9, 10, 11, 12, 13, 14, 15, -1, -1, 16, 17, 18, 19, 20, 21, 22, 23, -1,
    -1, 24, 25, 26, 27, 28, 29, 30, 31, -1, -1, 32, 33, 34, 35, 36, 37, 38, 39, -1, -1, 40, 41, 42,
    43, 44, 45, 46, 47, -1, -1, 48, 49, 50, 51, 52, 53, 54, 55, -1, -1, 56, 57, 58, 59, 60, 61, 62,
    63, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];

pub const MAIL_BOX_64: [i8; 64] = [
    21, 22, 23, 24, 25, 26, 27, 28, 31, 32, 33, 34, 35, 36, 37, 38, 41, 42, 43, 44, 45, 46, 47, 48,
    51, 52, 53, 54, 55, 56, 57, 58, 61, 62, 63, 64, 65, 66, 67, 68, 71, 72, 73, 74, 75, 76, 77, 78,
    81, 82, 83, 84, 85, 86, 87, 88, 91, 92, 93, 94, 95, 96, 97, 98,
];

pub enum Players {
    White,
    Black,
}

pub struct ChessBoard {
    board: [i8; BOARD_WIDTH * BOARD_HEIGHT],
    side_to_move: Players,
    castling_ability: [bool; 4], /* WKing, WQueen, BKing, BQueen */
    en_passant_target_square: Option<i8>,
    half_move_clock: u32,
    full_move_counter: u32,
}

pub struct Move {
    pub dx: i8,
    pub dy: i8,
}

pub fn test_hello() {
    println!("Hello test!");
}

impl Fen for ChessBoard {
    fn set_fen_position(fen: &str) -> Result<ChessBoard, FenError> {
        let fen_type = Self::validate_fen(fen)?;

        let mut fen_state = fen.split(" ");

        let board = parse_fen_part(&mut fen_state, parse_position, FenArguments::Position)?;

        let side_to_move =
            parse_fen_part(&mut fen_state, parse_side_to_move, FenArguments::SideToMove)?;

        let castling_ability = parse_fen_part(
            &mut fen_state,
            parse_castling_ability,
            FenArguments::CastlingAbility,
        )?;

        let en_passant_target_square = parse_fen_part(
            &mut fen_state,
            parse_epawn,
            FenArguments::EnPassantTargetSquare,
        )?;

        if fen_type == FenType::NoCounter {
            return Ok(Self {
                board,
                side_to_move,
                castling_ability,
                en_passant_target_square,
                half_move_clock: 0,
                full_move_counter: 0,
            });
        }

        let half_move_clock = parse_fen_part(
            &mut fen_state,
            parse_string_to_num,
            FenArguments::HalfMoveClock,
        )?;

        let full_move_counter = parse_fen_part(
            &mut fen_state,
            parse_string_to_num,
            FenArguments::FullMoveCounter,
        )?;

        Ok(Self {
            board,
            side_to_move,
            castling_ability,
            en_passant_target_square,
            half_move_clock,
            full_move_counter,
        })
    }
}

impl Default for ChessBoard {
    fn default() -> Self {
        Self::set_fen_position(FEN_STARTING_POSITION)
            .expect("Starting position should be valid and parsable!")
    }
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
