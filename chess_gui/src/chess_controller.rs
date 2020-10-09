use multiplayer::{chess_communicator, message::Message, OnlineConnection};
use piston::input::{Button, GenericEvent, MouseButton};
use rust_chess::{units::Color, *};
use std::sync::mpsc::{Receiver, Sender};

pub struct ChessController {
    pub chess_board: rust_chess::board::Board,
    pub game_over: bool,
    pub board_string: [[String; 8]; 8],
    pub selected_space: Option<(u8, u8)>,
    pub highlighted_spaces: Option<Vec<(i64, i64)>>,
    pub mouse_pos: [f64; 2],
    pub castling_possible: Vec<(String, (i64, i64))>,
    pub online_connection: Option<OnlineConnection<[u8; 5]>>,
    pub player_color: Option<Color>,
}

impl ChessController {
    pub fn new(
        chess_board: rust_chess::board::Board,
        online_connection: Option<OnlineConnection<[u8; 5]>>,
        player_color: Option<Color>,
    ) -> ChessController {
        let mut controller = ChessController {
            chess_board,
            game_over: false,
            board_string: Default::default(),
            selected_space: None,
            highlighted_spaces: None,
            mouse_pos: [0.0; 2],
            castling_possible: Vec::new(),
            online_connection,
            player_color,
        };
        controller.update_board();
        controller
    }

    fn update_board(&mut self) {
        for y in 0..8 {
            for x in 0..8 {
                let piece_str = match self.chess_board.get_square(x, y).piece.variety {
                    units::Variety::Empty => " ",
                    units::Variety::Pawn => "pawn",
                    units::Variety::Bishop => "bishop",
                    units::Variety::Knight => "knight",
                    units::Variety::Rook => "rook",
                    units::Variety::Queen => "queen",
                    units::Variety::King => "king",
                };
                if self.chess_board.get_square(x, y).piece.color.forward() == -1 {
                    let piece_str: String = "white_".to_string() + piece_str;
                    self.board_string[y][x] = piece_str;
                } else {
                    let piece_str: String = "black_".to_string() + piece_str;
                    self.board_string[y][x] = piece_str;
                }
            }
        }
    }

    fn exec_move(&mut self, mov: &str) {
        if let Some(connection) = &self.online_connection {
            connection
                .0
                .send(chess_communicator::move_to_bytes(mov.to_string()));
            if let Ok(message) = connection.1.recv() {
                if Message::Accept == message[0] {
                    self.game_over = self.chess_board.make_move(mov).0;
                    self.update_board();
                } else {
                    println!("move not accepted by opponent!");
                }
            } else {
                println!("error recieving return message after move");
            }
        } else {
            self.game_over = self.chess_board.make_move(mov).0;
            self.update_board();
        }
    }

    pub fn event<E: GenericEvent>(
        &mut self,
        view_pos: [f64; 2],
        view_size: f64,
        view_grid_width: f64,
        event: &E,
    ) {
        if let Some(pos) = event.mouse_cursor_args() {
            self.mouse_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            if !self.game_over {
                if let Some(connection) = &mut self.online_connection {
                    if let Ok(message) = connection.1.try_recv() {
                        if Message::Move == message[0] {
                            let mov = chess_communicator::process_move(&message).unwrap();
                            println!("{}", mov);
                            if !self.chess_board.make_move(&mov).1 {
                                connection
                                    .0
                                    .send([Message::Decline as u8; 5])
                                    .expect("error sending move decline message!")
                            } else {
                                connection
                                    .0
                                    .send([Message::Accept as u8; 5])
                                    .expect("error sending move accept message!")
                            }
                        } else {
                            self.update_board();
                        }
                    }
                }
                let (x, y) = (
                    self.mouse_pos[0] - view_pos[0],
                    self.mouse_pos[1] - view_pos[1],
                );
                let (int_x, int_y) = ((x / view_size * 8.0) as u8, (y / view_size * 8.0) as u8);
                if x > 0.0
                    && x < view_size
                    && y > 0.0
                    && y < view_size
                    && (x - int_x as f64 * ((view_size - view_grid_width) / 8.0)) > view_grid_width
                    && (y - int_y as f64 * ((view_size - view_grid_width) / 8.0)) > view_grid_width
                    && (x - (int_x + 1) as f64 * ((view_size - view_grid_width) / 8.0)) < 0.0
                    && (y - (int_y + 1) as f64 * ((view_size - view_grid_width) / 8.0)) < 0.0
                {
                    let (x, y) = (int_x, 7 - int_y);
                    if let Some(highlighted_spaces) = &self.highlighted_spaces {
                        for spaces in highlighted_spaces {
                            if x == spaces.0 as u8 && y == 7 - spaces.1 as u8 {
                                //castling workaround!
                                for castling in self.castling_possible.to_owned() {
                                    if x == (castling.1).0 as u8 && y == 7 - (castling.1).1 as u8 {
                                        self.exec_move(castling.0.as_str());
                                        break;
                                    }
                                }
                                let mut input: String = rust_chess::board::position_to_string(
                                    self.selected_space.expect("no inital piece selection").1,
                                    self.selected_space.expect("no inital piece selection").0,
                                ) + " ";
                                input
                                    .push_str(rust_chess::board::position_to_string(y, x).as_str());

                                if let rust_chess::units::Variety::Pawn = self
                                    .chess_board
                                    .get_square(
                                        self.selected_space.expect("no inital piece selection").0
                                            as usize,
                                        7 - self
                                            .selected_space
                                            .expect("no inital piece selection")
                                            .1 as usize,
                                    )
                                    .piece
                                    .variety
                                {
                                    if y == 0 || y == 7 {
                                        input.push_str("=Q");
                                    }
                                }
                                self.exec_move(input.as_str());
                                break;
                            }
                        }
                        self.castling_possible = Vec::new();
                        self.selected_space = None;
                        self.highlighted_spaces = None;
                    } else {
                        self.selected_space = Some((x, y));
                        let pos = rust_chess::board::position_to_string(y, x);
                        self.chess_board.set_promotion(true);
                        let mut possible_moves = self.chess_board.get_moves(pos.as_str());
                        self.chess_board.set_promotion(false);
                        //Workaround for castling to work
                        if let rust_chess::units::Variety::King = self
                            .chess_board
                            .get_square(x as usize, 7 - y as usize)
                            .piece
                            .variety
                        {
                            let (valid, _, king_new_x, _, _, y) =
                                moves::kingside_castling(&mut self.chess_board);
                            if valid {
                                possible_moves.push((king_new_x as i64, y as i64));
                                self.castling_possible
                                    .push(("O-O".to_string(), (king_new_x as i64, y as i64)));
                            }
                            let (valid, _, king_new_x, _, _, y) =
                                moves::queenside_castling(&mut self.chess_board);
                            if valid {
                                possible_moves.push((king_new_x as i64, y as i64));
                                self.castling_possible
                                    .push(("O-O-O".to_string(), (king_new_x as i64, y as i64)));
                            }
                        } else {
                            self.castling_possible = Vec::new();
                        }
                        self.highlighted_spaces = Some(possible_moves);
                    }
                } else {
                    self.selected_space = None;
                    self.highlighted_spaces = None;
                    self.castling_possible = Vec::new();
                }
            }
        }
    }
}
