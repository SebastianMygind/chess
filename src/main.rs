mod chess_game;
mod chessboard;
mod engine;
mod fen;
mod moves;
mod tests;
use chess_game::ChessGame;

fn main() -> iced::Result {
    let mut game = ChessGame::default();

    game.run()
}
