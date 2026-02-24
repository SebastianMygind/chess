mod chess_game;
mod chessboard;
mod engine;
mod fen;
mod moves;
mod tests;
use chess_game::ChessGame;
use chessboard::ChessBoard;
use engine::ChessEngine;

const RUN_GAME: bool = false;

fn main() {
    let test_board = ChessBoard::default();

    let (specifics, moves) = test_board.perft(7);

    for (c_move, count) in specifics {
        println!("{}: {}", c_move, count)
    }

    println!("perft depth 7: {moves}");

    if RUN_GAME {
        let mut game = ChessGame::default();
        _ = game.run();
    }
}
