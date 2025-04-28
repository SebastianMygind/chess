use crate::chessboard::{MAIL_BOX_64, MAIL_BOX_120, Move};

const WPAWN_START_MOVES: [Move; 2] = [Move { dx: 0, dy: 1 }, Move { dx: 0, dy: 2 }];
const WPAWN_MOVE: Move = Move { dx: 0, dy: 1 };
const WPAWN_ATTACK_MOVES: [Move; 2] = [Move { dx: 1, dy: 1 }, Move { dx: -1, dy: 1 }];

const BPAWN_START_MOVES: [Move; 2] = [Move { dx: 0, dy: -1 }, Move { dx: 0, dy: -2 }];
const BPAWN_MOVE: Move = Move { dx: 0, dy: -1 };
const BPAWN_ATTACK_MOVES: [Move; 2] = [Move { dx: 1, dy: -1 }, Move { dx: -1, dy: -1 }];

const KNIGHT_MOVES: [Move; 8] = [
    Move { dx: 1, dy: 2 },
    Move { dx: 1, dy: -2 },
    Move { dx: 2, dy: 1 },
    Move { dx: 2, dy: -1 },
    Move { dx: -1, dy: 2 },
    Move { dx: -1, dy: -2 },
    Move { dx: -2, dy: 1 },
    Move { dx: -2, dy: -1 },
];

const DIAGONAL_MOVES: [Move; 4] = [
    Move { dx: 1, dy: 1 },
    Move { dx: -1, dy: 1 },
    Move { dx: 1, dy: -1 },
    Move { dx: -1, dy: -1 },
];
const ANTI_DIAGONAL_MOVES: [Move; 4] = [
    Move { dx: 1, dy: 0 },
    Move { dx: -1, dy: 0 },
    Move { dx: 0, dy: 1 },
    Move { dx: 0, dy: -1 },
];
const ALL_DIRECTION_MOVES: [Move; 8] = concat_const_arrays(DIAGONAL_MOVES, ANTI_DIAGONAL_MOVES);

pub fn get_new_position(current_board_position: usize, chess_move: Move) -> Option<i8> {
    debug_assert!(current_board_position < 64);

    let current_mailbox_pos: i8 = MAIL_BOX_64[current_board_position];

    let new_position: i8 = current_mailbox_pos + chess_move.dx + (chess_move.dy * 10);

    let new_mail_box_pos: i8 = MAIL_BOX_120[new_position as usize];

    if new_mail_box_pos != -1 {
        return Some(new_mail_box_pos);
    }
    None
}

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
