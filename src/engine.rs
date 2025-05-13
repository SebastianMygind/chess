use moves::MoveType;

struct LegalMove {
    from: usize,
    to: usize,
    move_type: MoveType,
}

struct RatedMove {
    chess_move: LegalMove,
    rating: i32,
}

pub trait ChessEngine {
    fn legal_moves() -> Vec<LegalMove>;
}
