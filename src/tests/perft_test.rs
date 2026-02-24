// This module generally functions as an end-to-end test for the engine + fen parser.
// As these test compare against stockfish, which in all practical manners
// is the source of truth for move generation.

#[cfg(test)]
mod tests {
    use crate::{chessboard::ChessBoard, engine::ChessEngine, fen::Fen};

    const KIWI_PETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    const EN_PASSANT_1: &str = "4k3/8/8/Pp6/8/8/8/4K3 w - b6 0 1";
    const PERFT_POS_1: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    const PERFT_POS_2: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    const PERFT_POS_3: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    const PERFT_POS_4: &str =
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";

    #[test]
    fn start_pos_test() {
        let board = ChessBoard::default();

        let (_, leaf_count) = board.perft(4);

        assert_eq!(leaf_count, 197281);
    }

    #[test]
    fn kiwi_pete_test() {
        let board = ChessBoard::set_fen_position(KIWI_PETE).expect("Is valid FEN");

        let (_, leaf_count) = board.perft(4);

        assert_eq!(leaf_count, 4085603);
    }

    #[test]
    fn en_passant_position_1() {
        let board = ChessBoard::set_fen_position(EN_PASSANT_1).expect("Is valid FEN");

        let (_, leaf_count) = board.perft(4);

        assert_eq!(leaf_count, 2211);
    }

    #[test]
    fn perft_test_1() {
        let board = ChessBoard::set_fen_position(PERFT_POS_1).expect("Is valid FEN");

        let (_, leaf_count) = board.perft(4);

        assert_eq!(leaf_count, 43238);
    }

    #[test]
    fn perft_test_2() {
        let board = ChessBoard::set_fen_position(PERFT_POS_2).expect("Is valid FEN");

        let (_, leaf_count) = board.perft(4);

        assert_eq!(leaf_count, 422333);
    }

    #[test]
    fn perft_test_3() {
        let board = ChessBoard::set_fen_position(PERFT_POS_3).expect("Is valid FEN");

        let (_, leaf_count) = board.perft(4);

        assert_eq!(leaf_count, 2103487);
    }

    #[test]
    fn perft_test_4() {
        let board = ChessBoard::set_fen_position(PERFT_POS_4).expect("Is valid FEN");

        let (_, leaf_count) = board.perft(4);

        assert_eq!(leaf_count, 3894594);
    }
}
