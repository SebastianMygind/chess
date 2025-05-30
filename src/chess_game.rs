use crate::chessboard::{
    BBISHOP, BKING, BKNIGHT, BPAWN, BQUEEN, BROOK, ChessBoard, EMPTY, Players, WBISHOP, WKING,
    WKNIGHT, WPAWN, WQUEEN, WROOK,
};
use crate::engine::ChessEngine;
use crate::moves::MoveType;
use iced::widget::{Row, button, column, container, row, svg, text};
use iced::{self, ContentFit, Event, Length, Pixels};
use iced::{Element, Fill, Task};

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Quit,
    Reset,
    ClickedSquare(usize),
    Event(Event),
    SwitchPerspective,
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
    window_size: Option<iced::Size>,
}

impl Default for ChessGame {
    fn default() -> Self {
        Self {
            game: None,
            perspective: Players::White,
            selected_square: None,
            piece_sprite: SvgPieces::default(),
            window_size: None,
        }
    }
}

impl ChessGame {
    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::event::listen().map(Message::Event)
    }

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
                        if from == square {
                            self.selected_square = None;
                            return Task::none();
                        }

                        for legal_move in game.legal_moves().iter() {
                            match legal_move.move_type {
                                MoveType::PawnMove {
                                    promotion_move: Some(_prom_move),
                                } => {
                                    if legal_move.from == from && legal_move.to == square {
                                        todo!("Implement a way to choose promotion in gui.")
                                    }
                                }

                                _ => {
                                    if legal_move.from == from && legal_move.to == square {
                                        game.make_move(*legal_move);
                                        self.game = Some(game);
                                        self.selected_square = None;
                                        return Task::none();
                                    }
                                }
                            }
                        }
                        Task::none()
                    }
                },
            },
            Message::Quit => iced::exit(),

            Message::Event(event) => match event {
                Event::Window(window_event) => match window_event {
                    iced::window::Event::Resized(size) => {
                        self.window_size = Some(size);
                        iced::Task::none()
                    }
                    _ => iced::Task::none(),
                },
                _ => iced::Task::none(),
            },
            Message::SwitchPerspective => {
                let perspective = self.perspective;

                match perspective {
                    Players::White => self.perspective = Players::Black,
                    Players::Black => self.perspective = Players::White,
                }
                iced::Task::none()
            }
        }
    }

    pub fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.game {
            Some(_) => {
                if let Some(window_size) = self.window_size {
                    let scale = 0.8;

                    let board_size = if window_size.height > window_size.width {
                        window_size.width * scale
                    } else {
                        window_size.height * scale
                    };

                    let square_length = iced::Length::Fixed(board_size / 8.);
                    let board_length = iced::Length::Fixed(board_size);

                    let top_bar: iced::widget::Container<Message> = container(row![
                        button(text("reset board")).on_press(Message::Reset),
                        button(text("switch perspective")).on_press(Message::SwitchPerspective)
                    ]);

                    let board = render_board(&self, square_length);

                    column![
                        top_bar,
                        container(board)
                            .height(board_length)
                            .width(board_length)
                            .center(Fill)
                    ]
                    .into()
                } else {
                    text!("Waiting for window size!").into()
                }
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
            .subscription(ChessGame::subscription)
            .run()
    }
}

fn render_board(state: &ChessGame, square_size: Length) -> iced::widget::Column<'_, Message> {
    let mut board_columns = iced::widget::Column::new();
    let mut board_rows = iced::widget::Row::new();

    let board_state = state
        .game
        .to_owned()
        .expect("Expect the board here from callin function!");

    for i in 0..64 {
        board_rows = board_rows.push(get_button_from_square(
            i,
            board_state.board[get_corrected_index(i, state.perspective)],
            &state.piece_sprite,
            state.perspective,
            state.selected_square,
            square_size,
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
    selected_square: Option<usize>,
    square_size: Length,
) -> iced::widget::Button<Message> {
    let correct_index = get_corrected_index(position, perspective);

    let button = match square {
        WKING | BKING | WQUEEN | BQUEEN | WROOK | BROOK | WBISHOP | BBISHOP | WKNIGHT | BKNIGHT
        | WPAWN | BPAWN => button(
            pieces
                .to_iced_svg(square)
                .width(square_size)
                .height(square_size)
                .content_fit(ContentFit::Cover),
        )
        .width(square_size)
        .height(square_size)
        .on_press(Message::ClickedSquare(correct_index)),

        EMPTY => button(text(" "))
            .width(square_size)
            .height(square_size)
            .on_press(Message::ClickedSquare(correct_index)),
        _ => unreachable!("not allowed as square type/value"),
    };

    if should_be_light_square(correct_index) {
        button.style(move |theme: &iced::Theme, status| {
            let palette = theme.palette();

            if selected_square.is_some() && selected_square.expect("msg") == correct_index {
                button::Style::default().with_background(palette.text.scale_alpha(0.5))
            } else {
                match status {
                    button::Status::Active => {
                        button::Style::default().with_background(palette.text)
                    }
                    button::Status::Hovered => {
                        button::Style::default().with_background(palette.text.scale_alpha(0.7))
                    }
                    _ => button::primary(theme, status),
                }
            }
        })
    } else {
        button.style(move |theme: &iced::Theme, status| {
            let palette = theme.extended_palette();

            if selected_square.is_some() && selected_square.expect("msg") == correct_index {
                button::Style::default().with_background(palette.primary.base.color)
            } else {
                match status {
                    button::Status::Active => {
                        button::Style::default().with_background(palette.primary.strong.color)
                    }

                    button::Status::Hovered => {
                        button::Style::default().with_background(palette.primary.weak.color)
                    }

                    _ => button::primary(theme, status),
                }
            }
        })
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
