pub const LEGAL_POS1: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";

pub const LEGAL_POS2: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

pub const KIWI_PETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fen::{FEN_STARTING_POSITION, Fen, FenArguments, FenError, FenErrorKind, FenType};

    struct TestFenConverter;

    impl Fen for TestFenConverter {
        fn set_fen_position(_fen: &str) -> Result<crate::chessboard::ChessBoard, FenError> {
            todo!()
        }
    }

    #[test]
    fn fen_splitter_test1() {
        let mut fen_state = FEN_STARTING_POSITION.split(" ");

        _ = fen_state.next();

        assert_eq!(fen_state.next(), Some("w"));
    }

    #[test]
    fn fen_splitter_test2() {
        let mut fen_state = FEN_STARTING_POSITION.split(" ");

        _ = fen_state.next();
        _ = fen_state.next();

        assert_eq!(fen_state.next(), Some("KQkq"));
    }

    #[test]
    fn fen_validator1() {
        assert_eq!(
            TestFenConverter::validate_fen(FEN_STARTING_POSITION),
            Ok(FenType::Full)
        );
    }
    #[test]
    fn fen_validator2() {
        assert_eq!(
            TestFenConverter::validate_fen(LEGAL_POS1),
            Ok(FenType::Full)
        );
    }
    #[test]
    fn fen_validator3() {
        assert_eq!(
            TestFenConverter::validate_fen(LEGAL_POS2),
            Ok(FenType::Full)
        );
    }
    #[test]
    fn fen_validator4() {
        assert_eq!(
            TestFenConverter::validate_fen(KIWI_PETE),
            Ok(FenType::NoCounter)
        );
    }
    #[test]
    fn fen_validator5() {
        let string_to_test = "rnbqkbnr/pppppppp/8/8/8/8/PPPhPPPP/RNBQKBNR w KQkq - 0 1";

        assert_eq!(
            TestFenConverter::validate_fen(string_to_test),
            Err(FenError::new(
                FenErrorKind::InvalidArgument(FenArguments::Position),
                String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPhPPPP/RNBQKBNR")
            ))
        )
    }

    #[test]
    fn fen_validator6() {
        assert_eq!(
            TestFenConverter::validate_fen(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR  KQkq - 0 1"
            ),
            Err(FenError::new(
                FenErrorKind::InvalidArgument(FenArguments::SideToMove),
                String::from("")
            ))
        )
    }

    #[test]
    fn fen_validator7() {
        assert_eq!(
            TestFenConverter::validate_fen("    "),
            Err(FenError::new(
                FenErrorKind::InvalidArgument(FenArguments::Position),
                String::from("")
            ))
        )
    }
}
