use crate::chessboard::Move;

pub const fn fold_pawn_moves(single: Move, double: Move, attack: [Move; 2]) -> [Move; 4] {
    return [single, double, attack[0], attack[1]];
}
