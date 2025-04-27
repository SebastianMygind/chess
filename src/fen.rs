use validation::{
    is_castling_valid, is_en_passant_valid, is_half_move_valid, is_move_counter_valid,
    is_position_valid, is_side_to_move_valid, validate_fen_part,
};

use crate::chessboard::ChessBoard;
use std::fmt::Formatter;

pub mod parsing;
mod validation;

#[derive(Debug, PartialEq)]
pub enum FenType {
    Full,
    NoCounter,
}

#[derive(Debug, PartialEq)]
pub enum FenArguments {
    Position,
    SideToMove,
    CastlingAbility,
    EnPassantTargetSquare,
    HalfMoveClock,
    FullMoveCounter,
}

#[derive(Debug, PartialEq)]
pub enum FenErrorKind {
    InvalidArgument(FenArguments),
    MissingArgument(FenArguments),
    TooManyArguments,
    ParserError(FenArguments),
}

#[derive(Debug)]
pub struct MoveError {}

#[derive(Debug, PartialEq)]
pub struct FenError {
    pub kind: FenErrorKind,
    pub given_string: String,
}

pub const FEN_STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub trait Fen {
    fn validate_fen(fen: &str) -> Result<FenType, FenError> {
        let mut fen_state = fen.split(" ");
        let fen_string = fen.to_string();

        validate_fen_part(&mut fen_state, FenArguments::Position, is_position_valid)?;

        validate_fen_part(
            &mut fen_state,
            FenArguments::SideToMove,
            is_side_to_move_valid,
        )?;

        validate_fen_part(
            &mut fen_state,
            FenArguments::CastlingAbility,
            is_castling_valid,
        )?;

        validate_fen_part(
            &mut fen_state,
            FenArguments::EnPassantTargetSquare,
            is_en_passant_valid,
        )?;

        match validate_fen_part(
            &mut fen_state,
            FenArguments::HalfMoveClock,
            is_half_move_valid,
        ) {
            Ok(_) => {}
            Err(e) => {
                return match e {
                    FenError {
                        kind: FenErrorKind::MissingArgument(FenArguments::HalfMoveClock),
                        given_string: fen_string,
                    } => Ok(FenType::NoCounter),
                    _ => return Err(e),
                };
            }
        }

        validate_fen_part(
            &mut fen_state,
            FenArguments::FullMoveCounter,
            is_move_counter_valid,
        )?;

        if fen_state.next().is_some() {
            return Err(FenError {
                kind: FenErrorKind::TooManyArguments,
                given_string: fen_string,
            });
        }

        Ok(FenType::Full)
    }
    fn set_fen_position(fen: &str) -> Result<ChessBoard, FenError>;
}

impl FenError {
    pub fn new(kind: FenErrorKind, given_string: String) -> Self {
        Self { kind, given_string }
    }
    pub fn missing_argument(argument: FenArguments, fen_string: String) -> Self {
        Self::new(FenErrorKind::MissingArgument(argument), fen_string)
    }

    pub fn parser_error(arguments: FenArguments, given_string: String) -> Self {
        Self::new(FenErrorKind::ParserError(arguments), given_string)
    }
}

impl std::fmt::Display for FenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
