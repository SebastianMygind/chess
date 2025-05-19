mod king;
mod pawn;
mod queen;
use king::get_king_moves;
use queen::get_queen_moves;

use crate::chessboard::{
    BBISHOP, BKING, BKNIGHT, BPAWN, BQUEEN, BROOK, ChessBoard, EMPTY, Move, Players, WBISHOP,
    WKING, WKNIGHT, WPAWN, WQUEEN, WROOK,
};
use crate::moves::{
    ALL_DIRECTION_MOVES, ANTI_DIAGONAL_MOVES, BPAWN_ATTACK_MOVES, DIAGONAL_MOVES, KNIGHT_MOVES,
    LegalMove, MoveType, RatedMove, WPAWN_ATTACK_MOVES,
};

pub trait ChessEngine {
    fn legal_moves(&self) -> Vec<LegalMove>;

    fn perft(&self) -> Vec<(String, u32)>;
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

    fn perft(&self) -> Vec<(String, u32)> {
        todo!()
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

            WROOK | BROOK => {}

            WBISHOP | BBISHOP => {}

            WKNIGHT | BKNIGHT => {}

            WPAWN => {}

            BPAWN => {}

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

            if target_square.is_positive() == (chessboard.side_to_move == Players::White) {
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

        if target_square.is_positive() == (chessboard.side_to_move == Players::White) {
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
