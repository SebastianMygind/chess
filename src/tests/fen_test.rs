pub const LEGAL_POS1: &str = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";

pub const LEGAL_POS2: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

pub const KIWI_PETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        chessboard::{
            BBISHOP, BKING, BKNIGHT, BPAWN, BQUEEN, BROOK, EMPTY, WBISHOP, WKING, WKNIGHT, WPAWN,
            WQUEEN, WROOK,
        },
        fen::{
            FEN_STARTING_POSITION, Fen, FenArguments, FenError, FenErrorKind, FenType,
            parsing::PositionIterator,
        },
    };

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

    #[test]
    fn fen_position_iterator() {
        let pos_iter = PositionIterator {
            empty_remainder: Some(5),
            chars: "K".chars(),
        };

        let collected: Vec<i8> = pos_iter.collect();

        assert_eq!(collected, vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WKING])
    }

    #[test]
    fn fen_position_iterator_with_slash() {
        let pos_iter = PositionIterator {
            empty_remainder: Some(5),
            chars: "/K".chars(),
        };

        let collected: Vec<i8> = pos_iter.collect();

        assert_eq!(collected, vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WKING])
    }

    #[test]
    fn test_pos_iter_correct_length() {
        let pos_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

        let pos_iter = PositionIterator {
            empty_remainder: None,
            chars: pos_fen.chars(),
        };

        let collected: Vec<i8> = pos_iter.collect();

        assert_eq!(collected.len(), 64)
    }

    #[test]
    fn test_pos_iter_with_start_pos() {
        let pos_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

        let pos_iter = PositionIterator {
            empty_remainder: None,
            chars: pos_fen.chars(),
        };

        let collected: Vec<i8> = pos_iter.collect();

        let mut corrected_collection: Vec<i8> = Vec::with_capacity(64);

        for i in (0..=7).rev() {
            for j in 0..=7 {
                corrected_collection.push(collected[i * 8 + j])
            }
        }

        let expected_board = vec![
            WROOK, WKNIGHT, WBISHOP, WQUEEN, WKING, WBISHOP, WKNIGHT, WROOK, WPAWN, WPAWN, WPAWN,
            WPAWN, WPAWN, WPAWN, WPAWN, WPAWN, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
            EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
            EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
            EMPTY, BPAWN, BPAWN, BPAWN, BPAWN, BPAWN, BPAWN, BPAWN, BPAWN, BROOK, BKNIGHT, BBISHOP,
            BQUEEN, BKING, BBISHOP, BKNIGHT, BROOK,
        ];

        assert_eq!(corrected_collection, expected_board)
    }
}
