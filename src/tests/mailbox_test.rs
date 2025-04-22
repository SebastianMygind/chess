#[cfg(test)]
mod tests {
    use crate::chessboard::Move;
    use crate::legal_moves::get_new_position;

    #[test]
    fn test_legal_move_1() {
        let current_position = 0;

        let move_to_make: Move = Move { dx: 0, dy: 1 };

        let new_pos = get_new_position(current_position, move_to_make);

        assert_eq!(new_pos, Some(8));
    }
}
