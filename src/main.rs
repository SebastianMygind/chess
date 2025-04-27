mod chessboard;
mod fen;
mod legal_moves;
mod tests;

use chessboard::ChessBoard;

fn main() {
    let chess_board = ChessBoard::default();
    println!("{chess_board}");
}
