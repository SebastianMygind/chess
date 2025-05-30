mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;
use bishop::get_bishop_moves;
use king::get_king_moves;
use knight::get_knight_moves;
use pawn::get_pawn_moves;
use queen::get_queen_moves;
use rook::get_rook_moves;

use crate::chessboard::{
    BBISHOP, BKING, BKNIGHT, BPAWN, BQUEEN, BROOK, ChessBoard, EMPTY, Move, Players, WBISHOP,
    WKING, WKNIGHT, WPAWN, WQUEEN, WROOK,
};
use crate::moves::{
    ANTI_DIAGONAL_MOVES, BPAWN_ATTACK_MOVES, DIAGONAL_MOVES, KNIGHT_MOVES, LegalMove, MoveType,
    WPAWN_ATTACK_MOVES,
};

pub trait ChessEngine {
    fn legal_moves(&self) -> Vec<LegalMove>;

    fn perft(&self, depth: u32) -> (Vec<(String, u32)>, u32);
}

impl ChessEngine for ChessBoard {
    fn legal_moves(&self) -> Vec<LegalMove> {
        let mut legal_moves: Vec<LegalMove> = Vec::with_capacity(20);

        let pseudo_legal_moves = get_pseudo_legal_moves(self);

        for pseudo_legal_move in pseudo_legal_moves {
            let mut new_board = self.clone();
            new_board.make_move(pseudo_legal_move);

            let king_pos = if new_board.side_to_move == Players::White {
                new_board.black_king_position
            } else {
                new_board.white_king_position
            };

            if !king_is_checked(&new_board.board, king_pos) {
                legal_moves.push(pseudo_legal_move);
            }
        }

        legal_moves
    }

    fn perft(&self, depth: u32) -> (Vec<(String, u32)>, u32) {
        if depth == 0 {
            return (vec![(String::from("Depth = 0"), 0)], 0);
        } else if depth == 1 {
            let moves = self.legal_moves();
            let mut result: Vec<(String, u32)> = Vec::new();

            for legal in moves.iter() {
                let from = legal.from;
                let to = legal.to;

                result.push((format!("{from}, {to}"), 1));
            }

            let move_count = if moves.len() == 0 {
                1
            } else {
                moves.len() as u32
            };

            return (result, move_count);
        } else {
            let moves = self.legal_moves();
            let mut result: Vec<(String, u32)> = Vec::new();
            let mut move_count: u32 = 0;

            for legal in moves {
                let from = legal.from;
                let to = legal.to;

                let mut new_board = self.clone();

                new_board.make_move(legal);
                let (_, leaf_count) = new_board.perft(depth - 1);
                move_count += leaf_count;
                result.push((format!("{from}, {to}"), leaf_count));
            }
            return (result, move_count);
        }
    }
}

fn get_pseudo_legal_moves(chessboard: &ChessBoard) -> Vec<LegalMove> {
    let mut legal_moves: Vec<LegalMove> = Vec::with_capacity(30);

    for (index, square) in chessboard.board.iter().enumerate() {
        if *square == EMPTY {
            continue;
        }

        if !is_owned_piece(*square, chessboard.side_to_move) {
            continue;
        }

        match *square {
            WKING | BKING => {
                legal_moves.append(&mut get_king_moves(index, chessboard));
            }
            WQUEEN | BQUEEN => {
                legal_moves.append(&mut get_queen_moves(index, chessboard));
            }

            WROOK | BROOK => {
                legal_moves.append(&mut get_rook_moves(index, chessboard));
            }

            WBISHOP | BBISHOP => {
                legal_moves.append(&mut get_bishop_moves(index, chessboard));
            }

            WKNIGHT | BKNIGHT => {
                legal_moves.append(&mut get_knight_moves(index, chessboard));
            }

            WPAWN | BPAWN => {
                legal_moves.append(&mut get_pawn_moves(index, chessboard));
            }

            EMPTY => {
                continue;
            }

            _ => unreachable!("All cases have been covered!"),
        }
    }

    legal_moves
}

fn is_owned_piece(piece: i8, current_side: Players) -> bool {
    if current_side == Players::White {
        piece.is_positive()
    } else {
        piece.is_negative()
    }
}

pub fn get_multi_step_pseudo_legal_moves(
    chessboard: &ChessBoard,
    move_data: &[Move],
    position: usize,
    meta_data: MoveType,
) -> Vec<LegalMove> {
    let mut legal_moves: Vec<LegalMove> = Vec::with_capacity(10);

    for move_direction in move_data {
        let mut position_opt = move_direction.get_new_position(position);

        while position_opt.is_some() {
            let new_position = position_opt.expect("checked");
            position_opt = move_direction.get_new_position(new_position);

            let target_square = chessboard.board[new_position];

            let white_is_side_to_move = chessboard.side_to_move == Players::White;

            if target_square.is_positive() == white_is_side_to_move {
                break;
            }

            let is_capture = !(target_square == EMPTY);

            legal_moves.push(LegalMove {
                from: position,
                to: new_position,
                move_type: meta_data,
                is_capture,
            });

            if is_capture {
                break;
            }
        }
    }

    legal_moves
}

pub fn single_step_get_pseudo_legal_moves(
    chessboard: &ChessBoard,
    move_data: &[Move],
    position: usize,
    meta_data: MoveType,
) -> Vec<LegalMove> {
    let mut legal_moves: Vec<LegalMove> = Vec::with_capacity(4);

    for move_direction in move_data {
        let new_position = if let Some(pos) = move_direction.get_new_position(position) {
            pos
        } else {
            continue;
        };

        let target_square = chessboard.board[new_position];

        let white_is_side_to_move = chessboard.side_to_move == Players::White;

        if target_square.is_positive() == white_is_side_to_move {
            continue;
        }

        let is_capture = !(target_square == EMPTY);

        legal_moves.push(LegalMove {
            from: position,
            to: new_position,
            move_type: meta_data,
            is_capture,
        });
    }

    legal_moves
}

fn king_is_checked(board: &[i8; 64], king_position: usize) -> bool {
    /* Check per individual pieces, i.e. Diagonal moves: check only Queen, Bishop.. Anti-diagonal moves: check only Queen, Rook.. Pawn attacks */

    king_is_attacked_by_pawns(board, king_position)
        || king_is_attacked_on_diagonals(board, king_position)
        || king_is_attacked_on_anti_diagonals(board, king_position)
        || king_is_attacked_by_knights(board, king_position)
}

pub fn king_is_attacked_by_pawns(board: &[i8; 64], king_position: usize) -> bool {
    let (attack_moves, pawn) = if board[king_position].is_positive() {
        (WPAWN_ATTACK_MOVES, BPAWN)
    } else {
        (BPAWN_ATTACK_MOVES, WPAWN)
    };

    for attack_move in attack_moves {
        if let Some(pos) = attack_move.get_new_position(king_position) {
            if pawn == board[pos] {
                return true;
            }
        }
    }

    false
}

pub fn king_is_attacked_on_diagonals(board: &[i8; 64], king_position: usize) -> bool {
    /* No need to check for anything other than queen and bishops of opposing colors */
    let moves = DIAGONAL_MOVES;

    let danger_pieces = if board[king_position].is_positive() {
        [BQUEEN, BBISHOP]
    } else {
        [WQUEEN, WBISHOP]
    };

    for attack_move in moves {
        let mut pos_opt = attack_move.get_new_position(king_position);

        while pos_opt.is_some() {
            let new_pos = pos_opt.expect("prechecked for some variant");
            pos_opt = attack_move.get_new_position(new_pos);

            let square = board[new_pos];

            if square == EMPTY {
                continue;
            }

            if danger_pieces.contains(&square) {
                return true;
            } else {
                break;
            }
        }
    }

    false
}

pub fn king_is_attacked_on_anti_diagonals(board: &[i8; 64], king_position: usize) -> bool {
    /* No need to check for anything other than queen and bishops of opposing colors */
    let moves = ANTI_DIAGONAL_MOVES;

    let danger_pieces = if board[king_position].is_positive() {
        [BQUEEN, BROOK]
    } else {
        [WQUEEN, WROOK]
    };

    for attack_move in moves {
        let mut pos_opt = attack_move.get_new_position(king_position);

        while pos_opt.is_some() {
            let new_pos = pos_opt.expect("prechecked for some variant");
            pos_opt = attack_move.get_new_position(new_pos);

            let square = board[new_pos];

            if square == EMPTY {
                continue;
            }

            if danger_pieces.contains(&square) {
                return true;
            } else {
                break;
            }
        }
    }

    false
}

pub fn king_is_attacked_by_knights(board: &[i8; 64], king_position: usize) -> bool {
    let knight = if board[king_position].is_positive() {
        BKNIGHT
    } else {
        WKNIGHT
    };

    for attack_move in KNIGHT_MOVES {
        if let Some(pos) = attack_move.get_new_position(king_position) {
            if knight == board[pos] {
                return true;
            }
        }
    }

    false
}

pub enum MoveStatus {
    NoMove,
    Move,
    CaptureMove,
}

pub fn can_move_to_position(side_to_move: Players, square: i8) -> MoveStatus {
    if side_to_move == Players::White {
        if square == EMPTY {
            MoveStatus::Move
        } else if square.is_positive() {
            MoveStatus::NoMove
        } else {
            MoveStatus::CaptureMove
        }
    } else {
        if square == EMPTY {
            MoveStatus::Move
        } else if square.is_negative() {
            MoveStatus::NoMove
        } else {
            MoveStatus::CaptureMove
        }
    }
}

pub fn is_en_passant_capture(chessboard: &ChessBoard, new_position: usize) -> bool {
    if let Some(en_passant) = chessboard.en_passant_target_square {
        let target_square = if chessboard.side_to_move == Players::White {
            en_passant + 8
        } else {
            en_passant - 8
        };

        if target_square == new_position {
            return true;
        }
    }
    false
}

pub fn check_promotion_and_generate_moves(
    legal_move: LegalMove,
    promotion_pieces: &[i8],
) -> Vec<LegalMove> {
    let mut pawn_moves = Vec::new();

    if legal_move.to / 8 == 0 || legal_move.to / 8 == 7 {
        for piece in promotion_pieces {
            let mut promotion_move = legal_move.clone();
            promotion_move.move_type = MoveType::PawnMove {
                promotion_move: Some(*piece),
            };
            pawn_moves.push(promotion_move);
        }
    } else {
        pawn_moves.push(legal_move);
    }

    pawn_moves
}
