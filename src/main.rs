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

const RUN_GAME: bool = true;

fn main() {
    let depth = 5;

    let test_board = ChessBoard::set_fen_position("4k3/8/8/8/8/8/8/4K1N1 b - -").unwrap();
    let (specifics, moves) = test_board.perft(depth);

    println!("perft depth {depth}: {moves}");

    for (c_move, count) in specifics {
        println!("{}: {}", c_move, count)
    }

    if RUN_GAME {
        let mut game = ChessGame::default();
        _ = game.run();
    }
}
