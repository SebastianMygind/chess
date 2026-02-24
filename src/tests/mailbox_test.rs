#[cfg(test)]
mod tests {
    use crate::chessboard::Move;

    #[test]
    fn test_legal_move_1() {
        let current_position = 0;

        let move_to_make: Move = Move { dx: 0, dy: 1 };

        let new_pos = move_to_make.get_new_position(current_position);

        assert_eq!(new_pos, Some(8));
    }

    #[test]
    fn test_legal_move_2() {
        let current_position = 46;

        let move_to_make = Move { dx: 1, dy: 2 };

        let new_pos = move_to_make.get_new_position(current_position);

        assert_eq!(new_pos, Some(63));
    }

    #[test]
    fn test_legal_move_3() {
        let current_position = 61;

        let move_to_make = Move { dx: 1, dy: -1 };

        let new_pos = move_to_make.get_new_position(current_position);

        assert_eq!(new_pos, Some(54));
    }

    #[test]
    fn test_illegal_move_1() {
        let current_position = 0;

        let move_to_make = Move { dx: -1, dy: 0 };

        let new_pos = move_to_make.get_new_position(current_position);

        assert_eq!(new_pos, None);
    }

    #[test]
    fn test_illegal_move_2() {
        let current_position = 0;

        let move_to_make = Move { dx: 0, dy: -1 };

        let new_pos = move_to_make.get_new_position(current_position);

        assert_eq!(new_pos, None);
    }

    #[test]
    fn test_illegal_move_3() {
        let current_position = 63;

        let move_to_make = Move { dx: 1, dy: 2 };

        let new_pos = move_to_make.get_new_position(current_position);

        assert_eq!(new_pos, None);
    }
}
