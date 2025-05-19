#[cfg(test)]
mod tests {
    use crate::chessboard::{ChessBoard, EMPTY, Players, WPAWN};
    use crate::fen::Fen;
    use crate::moves::{LegalMove, MoveType};

    #[test]
    fn test_update_normal_move_1() {
        let legal_move = LegalMove {
            from: 0,
            to: 45,
            move_type: MoveType::Normal,
            is_capture: false,
        };

        let mut chessboard = ChessBoard::set_fen_position("4k3/3p4/8/8/8/8/5P2/B3K3 w - - 0 1")
            .expect("normal_move_test");
        chessboard.make_move(legal_move);

        let mut expected_board = ChessBoard::set_fen_position("4k3/3p4/8/8/8/8/5P2/B3K3 w - - 0 1")
            .expect("normal_move_test");

        expected_board.board[45] = expected_board.board[0];
        expected_board.board[0] = EMPTY;
        expected_board.half_move_clock += 1;
        expected_board.side_to_move = Players::Black;

        assert_eq!(
            chessboard, expected_board,
            "\n{chessboard}\n\n{expected_board}"
        );
    }

    #[test]
    fn test_update_pawn_move_1() {
        let legal_move = LegalMove {
            from: 13,
            to: 21,
            move_type: MoveType::PawnMove {
                promotion_move: None,
            },
            is_capture: false,
        };

        let mut chessboard = ChessBoard::set_fen_position("4k3/3p4/8/8/8/8/5P2/4K3 w - - 0 1")
            .expect("test should be valid!");
        chessboard.make_move(legal_move);

        let mut expected_board = ChessBoard::set_fen_position("4k3/3p4/8/8/8/8/5P2/4K3 w - - 0 1")
            .expect("test should be valid!");

        expected_board.board[13] = EMPTY;
        expected_board.board[21] = WPAWN;
        expected_board.side_to_move = Players::Black;

        assert_eq!(
            chessboard, expected_board,
            "\n{chessboard}\n\n{expected_board}"
        );
    }

    #[test]
    fn test_update_double_pawn_move_1() {
        let legal_move = LegalMove {
            from: 8,
            to: 24,
            move_type: MoveType::PawnDoubleMove,
            is_capture: false,
        };

        let mut chessboard = ChessBoard::default();

        let mut expected_board = ChessBoard::default();

        expected_board.board[8] = EMPTY;
        expected_board.board[24] = WPAWN;
        expected_board.side_to_move = Players::Black;
        expected_board.en_passant_target_square = Some(24);

        chessboard.make_move(legal_move);

        assert_eq!(chessboard, expected_board);
    }
}
