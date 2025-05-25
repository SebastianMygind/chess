mod chess_game;
mod chessboard;
mod engine;
mod fen;
mod moves;
mod tests;
use chess_game::ChessGame;
use chessboard::ChessBoard;
use engine::ChessEngine;

fn main() -> iced::Result {
    let mut game = ChessGame::default();

    let depth = 2;

    let test_board = ChessBoard::default();
    let (_, moves) = test_board.perft(depth);

    println!("perft depth {depth}: {moves}");

    game.run()
}
