mod chess_game;
mod chessboard;
mod fen;
mod legal_moves;
mod tests;
use chess_game::ChessGame;

fn main() -> iced::Result {
    let mut game = ChessGame::default();

    game.run()
}
