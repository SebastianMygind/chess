mod chess_game;
mod chessboard;
mod engine;
mod fen;
mod moves;
mod tests;
use chess_game::ChessGame;

const RUN_GAME: bool = true;

fn main() {
    if RUN_GAME {
        let mut game = ChessGame::default();
        _ = game.run();
    }
}
