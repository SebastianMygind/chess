use crate::{
    chessboard::{
        BBISHOP, BKING, BKNIGHT, BOARD_HEIGHT, BOARD_WIDTH, BPAWN, BQUEEN, BROOK, EMPTY, Players,
        WBISHOP, WKING, WKNIGHT, WPAWN, WQUEEN, WROOK,
    },
    fen::{FenArguments, FenError, FenErrorKind},
};
use std::str::Chars;

pub fn parse_fen_part<'a, F, T, Iter>(
    fen_state: &mut Iter,
    parser: F,
    argument: FenArguments,
) -> Result<T, FenError>
where
    Iter: Iterator<Item = &'a str>,
    F: Fn(&str) -> Option<T>,
{
    let current_fen_part = fen_state
        .next()
        .expect("Validated fen should have fen_part!");

    match parser(current_fen_part) {
        Some(parsed_part) => Ok(parsed_part),
        None => Err(FenError::new(
            FenErrorKind::ParserError(argument),
            current_fen_part.to_string(),
        )),
    }
}

pub struct PositionIterator<'a> {
    pub empty_remainder: Option<u32>,
    pub chars: Chars<'a>,
}

impl<'a> PositionIterator<'a> {
    fn parse_next_char(&mut self) -> Option<i8> {
        if let Some(char) = self.chars.next() {
            if let Some(digit) = char.to_digit(10) {
                if digit > 1 {
                    self.empty_remainder = Some(digit - 1);
                }

                return Some(EMPTY);
            }

            match char {
                'P' => Some(WPAWN),
                'p' => Some(BPAWN),

                'N' => Some(WKNIGHT),
                'n' => Some(BKNIGHT),

                'B' => Some(WBISHOP),
                'b' => Some(BBISHOP),

                'R' => Some(WROOK),
                'r' => Some(BROOK),

                'Q' => Some(WQUEEN),
                'q' => Some(BQUEEN),

                'K' => Some(WKING),
                'k' => Some(BKING),

                '/' => self.parse_next_char(),

                _ => None,
            }
        } else {
            None
        }
    }
}

impl<'a> Iterator for PositionIterator<'a> {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(e_remainder) = self.empty_remainder {
            if e_remainder == 1 {
                self.empty_remainder = None;
            } else {
                self.empty_remainder = Some(e_remainder - 1);
            }
            Some(EMPTY)
        } else {
            self.parse_next_char()
        }
    }
}

pub fn parse_position(str_part: &str) -> Option<[i8; 64]> {
    let mut board = [EMPTY; BOARD_WIDTH * BOARD_HEIGHT];

    let mut position_iterator = PositionIterator {
        empty_remainder: None,
        chars: str_part.chars(),
    };

    for i in (0..BOARD_HEIGHT).rev() {
        for j in 0..BOARD_WIDTH {
            board[(i * 8) + j] = position_iterator.next()?;
        }
    }

    if position_iterator.next().is_some() {
        eprintln!("parsing error, pos_string has to many characters!");
    }

    Some(board)
}

pub fn parse_side_to_move(str_part: &str) -> Option<Players> {
    let mut chars = str_part.chars();

    let side_to_move = chars.next()?;

    match side_to_move {
        'w' => Some(Players::White),
        'b' => Some(Players::Black),
        _ => None,
    }
}

pub fn parse_castling_ability(str_part: &str) -> Option<[bool; 4]> {
    let mut castling_ability = [false; 4];

    for char in str_part.chars() {
        match char {
            'K' => castling_ability[0] = true,
            'Q' => castling_ability[1] = true,

            'k' => castling_ability[2] = true,
            'q' => castling_ability[3] = true,

            '-' => return Some([false; 4]),
            _ => return None,
        }
    }

    Some(castling_ability)
}

pub fn parse_epawn(str_part: &str) -> Option<Option<usize>> {
    let mut epawn = None;
    let mut chars = str_part.chars();

    let char = chars.next()?;

    if char != '-' {
        let file = parse_file(char)?;
        let rank = parse_rank(chars.next()?)?;

        epawn = Some((rank * 8) + file);
    }

    Some(epawn)
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
