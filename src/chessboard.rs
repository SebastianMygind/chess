use crate::fen::parsing::{
    parse_castling_ability, parse_epawn, parse_fen_part, parse_position, parse_side_to_move,
    parse_string_to_num,
};
use crate::fen::{FEN_STARTING_POSITION, Fen, FenArguments, FenError, FenType};
use crate::moves::{LegalMove, MoveType};
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Players {
    White,
    Black,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChessBoard {
    pub board: [i8; BOARD_WIDTH * BOARD_HEIGHT],
    pub white_king_position: usize,
    pub black_king_position: usize,
    pub side_to_move: Players,
    pub castling_ability: [bool; 4], /* WKing, WQueen, BKing, BQueen */
    pub en_passant_target_square: Option<usize>,
    pub half_move_clock: u32,
    pub full_move_counter: u32,
}

impl ChessBoard {
    pub fn make_move(&mut self, move_to_make: LegalMove) {
        match move_to_make.move_type {
            MoveType::Normal => {
                self.board[move_to_make.to] = self.board[move_to_make.from];
                self.board[move_to_make.from] = EMPTY;
                self.update_half_moves();
                self.reset_enpassant();
            }

            MoveType::PawnMove { promotion_move } => {
                let piece_to_set = match promotion_move {
                    None => self.board[move_to_make.from],
                    Some(piece) => piece,
                };

                self.board[move_to_make.to] = piece_to_set;
                self.board[move_to_make.from] = EMPTY;

                self.reset_enpassant();
                self.reset_half_moves();
            }

            MoveType::PawnDoubleMove => {
                self.board[move_to_make.to] = self.board[move_to_make.from];
                self.board[move_to_make.from] = EMPTY;

                self.set_enpassant(move_to_make.to);
                self.reset_half_moves();
            }

            MoveType::Enpassant { target_square } => {
                self.board[move_to_make.to] = self.board[move_to_make.from];
                self.board[move_to_make.from] = EMPTY;
                self.board[target_square] = EMPTY;

                self.reset_enpassant();
                self.reset_half_moves();
            }

            MoveType::RookMove => {
                self.board[move_to_make.to] = self.board[move_to_make.from];
                self.board[move_to_make.from] = EMPTY;

                self.update_half_moves();
                self.check_and_update_rook(move_to_make.from);
                self.reset_enpassant();
            }

            MoveType::KingMove => {
                if self.board[move_to_make.from].is_positive() {
                    self.white_king_position = move_to_make.to;
                } else {
                    self.black_king_position = move_to_make.to;
                }

                self.board[move_to_make.to] = self.board[move_to_make.from];
                self.board[move_to_make.from] = EMPTY;
                self.update_half_moves();
                self.reset_enpassant();
                self.disable_castle();
            }

            MoveType::CastleKingSide => {
                let rook_pos_before = move_to_make.from + 3;

                if self.board[move_to_make.from].is_positive() {
                    self.white_king_position = move_to_make.to;
                } else {
                    self.black_king_position = move_to_make.to;
                }

                self.board[move_to_make.from + 1] = self.board[rook_pos_before];
                self.board[rook_pos_before] = EMPTY;

                self.board[move_to_make.to] = self.board[move_to_make.from];
                self.board[move_to_make.from] = EMPTY;

                self.disable_castle();
                self.update_half_moves();
                self.reset_enpassant();
            }

            MoveType::CastleQueenSide => {
                let rook_pos_before = move_to_make.from - 4;

                self.board[move_to_make.from - 1] = self.board[rook_pos_before];
                self.board[rook_pos_before] = EMPTY;

                self.board[move_to_make.to] = self.board[move_to_make.from];
                self.board[move_to_make.from] = EMPTY;

                self.disable_castle();
                self.update_half_moves();
                self.reset_enpassant();
            }
        }

        if move_to_make.is_capture {
            self.reset_half_moves();
            self.check_and_update_rook(move_to_make.to);
        }

        self.update_full_move();
        self.update_side_to_move();
    }

    fn set_enpassant(&mut self, location: usize) {
        self.en_passant_target_square = Some(location);
    }

    fn reset_enpassant(&mut self) {
        self.en_passant_target_square = None;
    }

    fn reset_half_moves(&mut self) {
        self.half_move_clock = 0;
    }

    fn update_half_moves(&mut self) {
        self.half_move_clock += 1;
    }

    fn update_full_move(&mut self) {
        match self.side_to_move {
            Players::White => {}

            Players::Black => {
                self.full_move_counter += 1;
            }
        }
    }

    fn disable_castle(&mut self) {
        match self.side_to_move {
            Players::White => {
                self.castling_ability[0] = false;
                self.castling_ability[1] = false;
            }
            Players::Black => {
                self.castling_ability[2] = false;
                self.castling_ability[3] = false;
            }
        }
    }

    fn check_and_update_rook(&mut self, captured_square: usize) {
        match captured_square {
            0 => {
                self.castling_ability[1] = false;
            }

            7 => {
                self.castling_ability[0] = false;
            }

            56 => {
                self.castling_ability[3] = false;
            }

            63 => {
                self.castling_ability[2] = false;
            }

            _ => {}
        }
    }

    fn update_side_to_move(&mut self) {
        match self.side_to_move {
            Players::White => self.side_to_move = Players::Black,
            Players::Black => self.side_to_move = Players::White,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub dx: i8,
    pub dy: i8,
}

impl Move {
    pub fn get_new_position(&self, current_board_position: usize) -> Option<usize> {
        debug_assert!(current_board_position < 64);

        let current_mailbox_pos: i8 = MAIL_BOX_64[current_board_position];

        let new_position: i8 = current_mailbox_pos + self.dx + (self.dy * 10);

        let new_mail_box_pos: i8 = MAIL_BOX_120[new_position as usize];

        if new_mail_box_pos != -1 {
            return Some(new_mail_box_pos as usize);
        }
        None
    }
}

impl Fen for ChessBoard {
    fn set_fen_position(fen: &str) -> Result<ChessBoard, FenError> {
        let fen_type = Self::validate_fen(fen)?;

        let mut fen_state = fen.split(" ").filter(|str| !str.is_empty());

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
                white_king_position: find_first_matching_piece(&board, WKING)
                    .expect("A king should exist on validated FEN"),
                black_king_position: find_first_matching_piece(&board, BKING)
                    .expect("A king should exist on validated FEN"),
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
            white_king_position: find_first_matching_piece(&board, WKING)
                .expect("A king should exist on validated FEN"),
            black_king_position: find_first_matching_piece(&board, BKING)
                .expect("A king should exist on validated FEN"),
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

const T_LINE: &str = "┌—————┬—————┬—————┬—————┬—————┬—————┬—————┬—————┐\n";
const H_LINE: &str = "|—————|—————|—————|—————|—————|—————|—————|—————|\n";
const B_LINE: &str = "└—————┴—————┴—————┴—————┴—————┴—————┴—————┴—————┘\n";

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_strings = parse_chessboard_to_string(&self.board);

        write!(
            f,
            "{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}{}\n{}",
            T_LINE,
            rank_strings[7],
            H_LINE,
            rank_strings[6],
            H_LINE,
            rank_strings[5],
            H_LINE,
            rank_strings[4],
            H_LINE,
            rank_strings[3],
            H_LINE,
            rank_strings[2],
            H_LINE,
            rank_strings[1],
            H_LINE,
            rank_strings[0],
            B_LINE
        )
    }
}

fn parse_chessboard_to_string(board: &[i8]) -> Vec<String> {
    let mut printable_board = Vec::new();

    for rank in 0..=7 {
        let mut pieces: Vec<char> = Vec::new();

        for file in 0..=7 {
            let index = (rank * 8) + file;
            pieces.push(piece_to_char(board[index]));
        }

        let rank_string: String = format!(
            "|  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |  {}  |",
            pieces[0], pieces[1], pieces[2], pieces[3], pieces[4], pieces[5], pieces[6], pieces[7],
        );

        printable_board.push(rank_string);
    }

    printable_board
}

fn piece_to_char(piece: i8) -> char {
    match piece {
        WPAWN => 'P',
        BPAWN => 'p',

        WBISHOP => 'B',
        BBISHOP => 'b',

        WKNIGHT => 'N',
        BKNIGHT => 'n',

        WROOK => 'R',
        BROOK => 'r',

        WQUEEN => 'Q',
        BQUEEN => 'q',

        WKING => 'K',
        BKING => 'k',

        EMPTY => ' ',

        _ => unreachable!("Illegal character in board!"),
    }
}

pub fn find_first_matching_piece(board: &[i8; 64], piece_to_find: i8) -> Option<usize> {
    let matching = board
        .iter()
        .enumerate()
        .find(|(_, piece)| **piece == piece_to_find);

    if let Some((pos, _)) = matching {
        Some(pos)
    } else {
        None
    }
}
