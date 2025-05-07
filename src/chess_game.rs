use crate::chessboard::{
    BBISHOP, BKING, BKNIGHT, BPAWN, BQUEEN, BROOK, ChessBoard, EMPTY, Players, WBISHOP, WKING,
    WKNIGHT, WPAWN, WQUEEN, WROOK,
};
use iced;
use iced::widget::{Row, button, column, container, row, svg, text};
use iced::{Element, Fill};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Start,
    Reset,
    ClickedSquare(usize),
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

impl SvgPieces {
    pub fn to_iced_svg(&self, square: i8) -> iced::widget::Svg<'_> {
        let handle = svg::Handle::from_memory(match square {
            WKING => self.white_king,
            BKING => self.black_king,

            WQUEEN => self.white_queen,
            BQUEEN => self.black_queen,

            WROOK => self.white_rook,
            BROOK => self.black_rook,

            WBISHOP => self.white_bishop,
            BBISHOP => self.black_bishop,

            WKNIGHT => self.white_knight,
            BKNIGHT => self.black_knight,

            WPAWN => self.white_pawn,
            BPAWN => self.black_pawn,

            _ => unreachable!("Should always give valid peace to function!"),
        });

        svg(handle)
    }
}

pub struct ChessGame {
    game: Option<ChessBoard>,
    perspective: Players,
    selected_square: Option<usize>,
    piece_sprite: SvgPieces,
}

impl Default for ChessGame {
    fn default() -> Self {
        Self {
            game: None,
            perspective: Players::White,
            selected_square: None,
            piece_sprite: SvgPieces::default(),
        }
    }
}

impl ChessGame {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Start => self.game = Some(ChessBoard::default()),
            Message::Reset => self.game = Some(ChessBoard::default()),
            Message::ClickedSquare(square) => match self.selected_square {
                None => self.selected_square = Some(square),
                Some(from) => match self.game.clone() {
                    None => {
                        unreachable!("Should only be able to move when initialized!")
                    }
                    Some(mut game) => {
                        game.board[square] = game.board[from];
                        game.board[from] = 0;
                        self.game = Some(game);
                        self.selected_square = None;
                    }
                },
            },
        }
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.game {
            Some(game) => match &self.perspective {
                Players::White => {
                    let mut board_columns = iced::widget::Column::new();
                    let mut board_rows = iced::widget::Row::new();

                    for (i, square) in game.board.iter().enumerate() {
                        board_rows =
                            board_rows.push(get_button_from_square(i, *square, &self.piece_sprite));
                        if i % 8 == 7 {
                            board_columns = board_columns.push(board_rows);
                            board_rows = Row::new();
                        }
                    }

                    container(board_columns).height(Fill).into()
                }
                Players::Black => text("black pieces perspective: TODO!").into(),
            },
            None => {
                let starting_text = text("Starting screen!");

                let button = button(text("start game!")).on_press(Message::Start);

                column![starting_text, button].into()
            }
        }
    }

    pub fn run(&mut self) -> iced::Result {
        iced::application("Chess", ChessGame::update, ChessGame::view)
            .theme(ChessGame::theme)
            .run()
    }
}

fn get_button_from_square(
    position: usize,
    square: i8,
    pieces: &SvgPieces,
) -> iced::widget::Button<Message> {
    match square {
        WKING | BKING | WQUEEN | BQUEEN | WROOK | BROOK | WBISHOP | BBISHOP | WKNIGHT | BKNIGHT
        | WPAWN | BPAWN => {
            button(pieces.to_iced_svg(square)).on_press(Message::ClickedSquare(position))
        }

        EMPTY => button(text(" ")).on_press(Message::ClickedSquare(position)),
        _ => unreachable!("not allowed as square type/value"),
    }
}

pub fn should_be_light_square(position: usize) -> bool {
    let row = (position / 8) + 1;

    let col = (position % 8) + 1;

    if row % 2 == 0 {
        col % 2 != 0
    } else {
        col % 2 == 0
    }
}
