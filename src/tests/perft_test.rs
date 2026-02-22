// This module generally functions as an end-to-end test for the engine + fen parser.
// As these test compare against stockfish, which in all practical manners
// is the source of truth for move generation.

#[cfg(test)]
mod tests {
    use crate::{ChessBoard, engine::ChessEngine, fen::Fen};

    const KIWI_PETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    const EN_PASSANT_1: &str = "4k3/8/8/Pp6/8/8/8/4K3 w - b6 0 1";

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
}
