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

use crate::fen::FEN_STARTING_POSITION;

const RUN_GAME: bool = false;

fn main() {
    let depth = 4;

    let test_board = ChessBoard::set_fen_position(FEN_STARTING_POSITION).unwrap();
    println!("{:?}", test_board.en_passant_target_square);

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
