use crate::chessboard::{
    BBISHOP, BKING, BKNIGHT, BPAWN, BQUEEN, BROOK, ChessBoard, EMPTY, Players, WBISHOP, WKING,
    WKNIGHT, WPAWN, WQUEEN, WROOK,
};
use iced::widget::{Row, button, column, container, row, svg, text};
use iced::{self, Color, ContentFit, Pixels};
use iced::{Background, Border, Shadow};
use iced::{Element, Fill, Task};
type BStyle = iced::widget::button::Style;

struct WhiteSquareStyle;
struct BlackSquareStyle;

impl WhiteSquareStyle {
    fn style(_theme: &iced::Theme, status: button::Status) -> button::Style {
        let white_square: BStyle = BStyle {
            background: Some(Background::Color(Color {
                r: 240.,
                g: 217.,
                b: 181.,
                a: 1.,
            })),
            text_color: Color::default(),
            border: Border::default(),
            shadow: Shadow::default(),
        };
        let white_square_active: BStyle = BStyle {
            background: Some(Background::Color(Color {
                r: 240.,
                g: 217.,
                b: 181.,
                a: 0.4,
            })),
            text_color: Color::default(),
            border: Border::default(),
            shadow: Shadow::default(),
        };

        match status {
            button::Status::Active | button::Status::Hovered | button::Status::Pressed => {
                white_square_active
            }
            button::Status::Disabled => white_square,
        }
    }
}

impl BlackSquareStyle {
    fn style(_theme: &iced::Theme, status: button::Status) -> button::Style {
        let black_square: BStyle = BStyle {
            background: Some(Background::Color(Color {
                r: 181.,
                g: 136.,
                b: 99.,
                a: 0.,
            })),
            text_color: Color::default(),
            border: Border::default(),
            shadow: Shadow::default(),
        };

        let black_square_active: BStyle = BStyle {
            background: Some(Background::Color(Color {
                r: 181.,
                g: 136.,
                b: 99.,
                a: 0.4,
            })),
            text_color: Color::default(),
            border: Border::default(),
            shadow: Shadow::default(),
        };

        match status {
            button::Status::Active | button::Status::Hovered | button::Status::Pressed => {
                black_square_active
            }
            button::Status::Disabled => black_square,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Start,
    Quit,
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
    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Start | Message::Reset => {
                self.game = Some(ChessBoard::default());
                Task::none()
            }
            Message::ClickedSquare(square) => match self.selected_square {
                None => {
                    self.selected_square = Some(square);

                    Task::none()
                }
                Some(from) => match self.game.clone() {
                    None => {
                        unreachable!("Should only be able to move when initialized!")
                    }
                    Some(mut game) => {
                        game.board[square] = game.board[from];
                        game.board[from] = 0;
                        self.game = Some(game);
                        self.selected_square = None;
                        Task::none()
                    }
                },
            },
            Message::Quit => iced::exit(),
        }
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.game {
            Some(_) => {
                let board = render_board(&self);

                container(board).height(Fill).padding([50., 50.]).into()
            }
            None => {
                let starting_text = container(
                    text("Starting screen!")
                        .center()
                        .width(Fill)
                        .size(Pixels::from(60)),
                )
                .padding(iced::Padding::from([50., 50.]));

                let start_button = button(text("start game!"))
                    .width(Fill)
                    .height(Fill)
                    .on_press(Message::Start);

                let exit_button = button(text("Quit"))
                    .width(Fill)
                    .height(Fill)
                    .on_press(Message::Quit);

                container(column![
                    starting_text,
                    row![start_button, exit_button].spacing(10)
                ])
                .padding(iced::Padding::from([100., 100.]))
                .into()
            }
        }
    }

    pub fn run(&mut self) -> iced::Result {
        iced::application("Chess", ChessGame::update, ChessGame::view)
            .theme(ChessGame::theme)
            .run()
    }
}

fn render_board(state: &ChessGame) -> iced::widget::Column<'_, Message> {
    let mut board_columns = iced::widget::Column::new();
    let mut board_rows = iced::widget::Row::new();

    let board_state = state
        .game
        .clone()
        .expect("Expect the board here from callin function!");

    for i in 0..64 {
        board_rows = board_rows.push(get_button_from_square(
            i,
            board_state.board[get_corrected_index(i, state.perspective)],
            &state.piece_sprite,
            state.perspective,
        ));
        if i % 8 == 7 {
            board_columns = board_columns.push(board_rows);
            board_rows = Row::new();
        }
    }
    board_columns
}

fn get_button_from_square(
    position: usize,
    square: i8,
    pieces: &SvgPieces,
    perspective: Players,
) -> iced::widget::Button<Message> {
    let correct_index = get_corrected_index(position, perspective);

    let button = match square {
        WKING | BKING | WQUEEN | BQUEEN | WROOK | BROOK | WBISHOP | BBISHOP | WKNIGHT | BKNIGHT
        | WPAWN | BPAWN => button(
            pieces
                .to_iced_svg(square)
                .width(Fill)
                .height(Fill)
                .content_fit(ContentFit::Cover),
        )
        .width(Fill)
        .height(Fill)
        .on_press(Message::ClickedSquare(correct_index)),

        EMPTY => button(text(" "))
            .width(Fill)
            .height(Fill)
            .on_press(Message::ClickedSquare(correct_index)),
        _ => unreachable!("not allowed as square type/value"),
    };

    if should_be_light_square(correct_index) {
        button.style(WhiteSquareStyle::style)
    } else {
        button.style(BlackSquareStyle::style)
    }
}

fn get_corrected_index(index: usize, perspective: Players) -> usize {
    match perspective {
        Players::White => {
            let column = 7 - (index / 8);
            let row = index % 8;

            row + (column * 8)
        }
        Players::Black => {
            let column = index / 8;
            let row = index % 8;

            row + (column * 8)
        }
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
