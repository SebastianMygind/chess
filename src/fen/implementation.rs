use crate::{
    chessboard::{BOARD_HEIGHT, BOARD_WIDTH, EMPTY, Players},
    fen::{Fen, FenArguments, FenError, FenErrorKind},
};
use std::str::Split;

pub fn parse_fen_part<F, T>(
    fen_state: &mut Split<&str>,
    parser: F,
    argument: FenArguments,
) -> Result<T, FenError>
where
    F: Fn(&str) -> Option<T>,
{
    let current_fen_part = fen_state
        .next()
        .expect("Validated fen should havbe fen_part!");

    match parser(current_fen_part) {
        Some(parsed_part) => Ok(parsed_part),
        None => Err(FenError::new(
            FenErrorKind::ParserError(argument),
            current_fen_part.to_string(),
        )),
    }
}

pub fn parse_position(str_part: &str) -> Option<[i8; 64]> {
    let board = [EMPTY; BOARD_WIDTH * BOARD_HEIGHT];

    todo!()
}

pub fn parse_side_to_move(str_part: &str) -> Option<Players> {
    todo!()
}

pub fn parse_castling_ability(str_part: &str) -> Option<[bool; 4]> {
    todo!()
}

pub fn parse_epawn(str_part: &str) -> Option<Option<i8>> {
    todo!()
}

pub fn parse_file(char: char) -> Option<usize> {
    Some(match char {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return None,
    })
}

pub fn parse_rank(char: char) -> Option<usize> {
    Some(match char {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        _ => return None,
    })
}

pub fn parse_string_to_num(fen_part: &str) -> Option<u32> {
    Some(fen_part.parse().expect("Should be validated"))
}
