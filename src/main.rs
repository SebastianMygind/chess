mod chess_game;
mod chessboard;
mod engine;
mod fen;
mod moves;
mod tests;
use chess_game::ChessGame;
use chessboard::ChessBoard;
use engine::ChessEngine;
use fen::Fen;

const RUN_GAME: bool = false;

fn main() {
    let depth = 8;

    let test_board = ChessBoard::set_fen_position("4k3/8/8/8/8/8/8/4K1N1 b - -").unwrap();
    let (_, moves) = test_board.perft(depth);

    println!("perft depth {depth}: {moves}");

    if RUN_GAME {
        let mut game = ChessGame::default();
        _ = game.run();
    }
}
