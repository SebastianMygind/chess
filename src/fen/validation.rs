use super::{FenArguments, FenError, FenErrorKind};
use std::str::Split;
const VALID_POSITION_CHARS: [char; 12] =
    ['k', 'q', 'r', 'b', 'n', 'p', 'K', 'Q', 'R', 'B', 'N', 'P'];

const VALID_FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

pub fn validate_fen_part<F>(
    fen_state: &mut Split<&str>,
    fen_argument: FenArguments,
    validator: F,
) -> Result<(), FenError>
where
    F: Fn(&str) -> bool,
{
    if let Some(part_str) = fen_state.next() {
        if !validator(part_str) {
            Err(FenError::new(
                FenErrorKind::InvalidArgument(fen_argument),
                part_str.to_string(),
            ))
        } else {
            Ok(())
        }
    } else {
        Err(FenError::missing_argument(
            fen_argument,
            fen_state.collect(),
        ))
    }
}

pub fn is_position_valid(position: &str) -> bool {
    let ranks = position.split('/');
    let mut rank: u32 = 0;

    for rank_string in ranks {
        let mut file = 0;

        for char in rank_string.chars() {
            match char.to_digit(10) {
                Some(digit) => {
                    if digit == 0 {
                        return false;
                    }
                    file += digit;
                }

                None => {
                    if !VALID_POSITION_CHARS.contains(&char) {
                        return false;
                    }
                    file += 1;
                }
            }
        }
        if file != 8 {
            return false;
        }
        rank += 1;
    }

    if rank != 8 {
        return false;
    }
    true
}

pub fn is_side_to_move_valid(side_to_move: &str) -> bool {
    let mut chars = side_to_move.chars();

    match chars.next() {
        Some(c) => {
            if !(c == 'w' || c == 'b') {
                return false;
            }
        }
        None => return false,
    };

    if chars.next().is_some() {
        return false;
    }

    true
}

pub fn is_castling_valid(castling_ability: &str) -> bool {
    let chars = castling_ability.chars();
    let mut has_no_ability = false;
    let mut char_occurences: [u32; 4] = [0; 4]; //WKing, WQueen, BKing, BQueen

    for c in chars {
        match c {
            'K' => char_occurences[0] += 1,
            'Q' => char_occurences[1] += 1,
            'k' => char_occurences[2] += 1,
            'q' => char_occurences[3] += 1,
            '-' => {
                if !has_no_ability {
                    has_no_ability = true;
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }
    if has_no_ability {
        for count in char_occurences {
            if count != 0 {
                return false;
            }
        }
    } else {
        for count in char_occurences {
            if !(count == 1 || count == 0) {
                return false;
            }
        }
    }
    true
}

pub fn is_en_passant_valid(en_passant: &str) -> bool {
    let mut chars = en_passant.chars();
    let mut no_en_passant = false;

    if let Some(char) = chars.next() {
        match char {
            '-' => no_en_passant = true,
            _ => {
                if !VALID_FILES.contains(&char) {
                    return false;
                }
            }
        }
    } else {
        return false;
    }

    let rank_option = chars.next();

    if no_en_passant && rank_option.is_some() {
        return false;
    }

    if let Some(rank_char) = rank_option {
        let digit = match rank_char.to_digit(10) {
            Some(digit) => digit,
            None => return false,
        };
        if digit != 3 && digit != 6 {
            return false;
        }
    }
    true
}

pub fn is_half_move_valid(half_move: &str) -> bool {
    match half_move.parse::<u32>() {
        Ok(half_move) => {
            if half_move > 50 {
                return false;
            }
        }
        Err(_) => return false,
    }
    true
}

pub fn is_move_counter_valid(move_counter: &str) -> bool {
    move_counter.parse::<u32>().is_ok()
}
