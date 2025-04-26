use crate::chessboard::{MAIL_BOX_64, MAIL_BOX_120, Move};

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
