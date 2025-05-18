#[cfg(test)]
mod tests {
    use crate::{
        chessboard::{BKING, BPAWN, WKING, WPAWN},
        engine::king_is_attacked_by_pawns,
    };

    #[test]
    fn test_king_is_checked_by_pawn_1() {
        let mut board = [0; 64];

        board[0] = WKING;

        board[9] = BPAWN;

        let is_checked = king_is_attacked_by_pawns(&board, 0);

        assert_eq!(is_checked, true);
    }

    #[test]
    fn test_king_is_checked_by_pawn_2() {
        let mut board = [0; 64];

        board[1] = WKING;

        board[8] = BPAWN;

        let is_checked = king_is_attacked_by_pawns(&board, 1);

        assert_eq!(is_checked, true);
    }

    #[test]
    fn test_king_is_checked_by_pawn_3() {
        let mut board = [0; 64];

        board[63] = BKING;

        board[54] = BPAWN;

        let is_checked = king_is_attacked_by_pawns(&board, 63);

        assert_eq!(is_checked, false);
    }

    #[test]
    fn test_king_is_checked_by_pawn_4() {
        let mut board = [0; 64];

        board[63] = BKING;

        board[54] = WPAWN;

        let is_checked = king_is_attacked_by_pawns(&board, 63);

        assert_eq!(is_checked, true);
    }
}
