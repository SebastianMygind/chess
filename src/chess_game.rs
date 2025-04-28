use crate::chessboard::{ChessBoard, Players};
use iced;
use iced::widget::svg;
use iced::{Element, Fill};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Start,
    Reset,
    Move((usize, usize)),
}

pub struct SvgPieces {
    pub white_king: &'static [u8],
    pub white_queen: &'static [u8],
    pub white_rook: &'static [u8],
    pub white_bishop: &'static [u8],
    pub white_knight: &'static [u8],
    pub white_pawn: &'static [u8],
    pub black_king: &'static [u8],
    pub black_queen: &'static [u8],
    pub black_rook: &'static [u8],
    pub black_bishop: &'static [u8],
    pub black_knight: &'static [u8],
    pub black_pawn: &'static [u8],
}

impl Default for SvgPieces {
    fn default() -> Self {
        let white_king = include_bytes!("../pieces/cburnett/wK.svg");
        let white_queen = include_bytes!("../pieces/cburnett/wQ.svg");
        let white_rook = include_bytes!("../pieces/cburnett/wR.svg");
        let white_bishop = include_bytes!("../pieces/cburnett/wB.svg");
        let white_knight = include_bytes!("../pieces/cburnett/wN.svg");
        let white_pawn = include_bytes!("../pieces/cburnett/wP.svg");
        let black_king = include_bytes!("../pieces/cburnett/bK.svg");
        let black_queen = include_bytes!("../pieces/cburnett/bQ.svg");
        let black_rook = include_bytes!("../pieces/cburnett/bR.svg");
        let black_bishop = include_bytes!("../pieces/cburnett/bB.svg");
        let black_knight = include_bytes!("../pieces/cburnett/bN.svg");
        let black_pawn = include_bytes!("../pieces/cburnett/bP.svg");
        SvgPieces {
            white_king,
            white_queen,
            white_rook,
            white_bishop,
            white_knight,
            white_pawn,
            black_king,
            black_queen,
            black_rook,
            black_bishop,
            black_knight,
            black_pawn,
        }
    }
}

pub struct ChessGame {
    game: Option<ChessBoard>,
    perspective: Players,
    piece_sprite: SvgPieces,
}

impl Default for ChessGame {
    fn default() -> Self {
        Self {
            game: Some(ChessBoard::default()),
            perspective: Players::White,
            piece_sprite: SvgPieces::default(),
        }
    }
}

impl ChessGame {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Start => self.game = Some(ChessBoard::default()),
            Message::Reset => self.game = Some(ChessBoard::default()),
            Message::Move((from, to)) => match self.game.clone() {
                None => {
                    unreachable!("Should only be able to move when initialized!")
                }
                Some(mut game) => {
                    game.board[to] = game.board[from];
                    game.board[from] = 0;
                }
            },
        }
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    pub fn view(&self) -> Element<'_, Message> {
        let handle = svg::Handle::from_memory(self.piece_sprite.white_queen);

        let svg = svg(handle).height(Fill).width(Fill);

        svg.into()
    }

    pub fn run(&mut self) -> iced::Result {
        iced::application("Chess", ChessGame::update, ChessGame::view)
            .theme(ChessGame::theme)
            .run()
    }
}
