#![allow(clippy::all)]
use crate::moves;
use crate::units;

use std::fs;
#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub piece: units::Piece,
}

#[allow(warnings)]
impl Square {
    pub fn is_empty(&self) -> bool {
        match self.piece.variety {
            units::Variety::Empty => true,
            _ => false,
        }
    }
    pub fn get_empty() -> Square {
        Square {
            piece: units::Piece {
                variety: units::Variety::Empty,
                color: units::Color::Empty,
                has_moved: false,
            },
        }
    }
}

pub fn get_variety(variety: char) -> units::Variety {
    match variety {
        'p' | 'P' => units::Variety::Pawn,
        'r' | 'R' => units::Variety::Rook,
        'n' | 'N' => units::Variety::Knight,
        'b' | 'B' => units::Variety::Bishop,
        'q' | 'Q' => units::Variety::Queen,
        'k' | 'K' => units::Variety::King,
        _ => units::Variety::Empty,
    }
}

pub fn string_to_position(pos: &str) -> (usize, usize) {
    let column = pos.chars().nth(0).unwrap();
    let row = pos.chars().nth(1).unwrap();

    if (column as i64) - 97 < 0 || 8 - ((row as i64) - 48) < 0 {
        return (100, 100);
    }
    let x = (column as usize) - 97;
    let y = 8 - ((row as usize) - 48);
    (x, y)
}

pub fn position_to_string(r: u8, c: u8) -> String {
    let row = (r + 1).to_string();
    let col = ((c + 97) as char).to_string();
    let pos = [col, row].join("");
    pos
}

pub fn get_color(color: char) -> units::Color {
    let ascii_color = color as u32;
    match ascii_color {
        ascii_color if (ascii_color > 96) => units::Color::White,
        _ => units::Color::Black,
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Board {
    grid: [[Square; 8]; 8],
    current_player: units::Color,
    passant: (i64, i64),
    promotion: bool,
}

impl Board {
    pub fn init() -> Board {
        let empty_square = Square::get_empty();

        Board {
            grid: [[empty_square; 8]; 8],
            current_player: units::Color::White,
            passant: (-1, -1),
            promotion: false,
        }
    }

    pub fn get_square(&self, x: usize, y: usize) -> Square {
        self.grid[y][x]
    }

    pub fn get_current_player(&self) -> units::Color {
        self.current_player
    }

    pub fn get_passant(&self) -> (i64, i64) {
        self.passant
    }

    //Needed for promotion to show up in the GUI, sadly the least intrusive way of doing so
    pub fn set_promotion(&mut self, val: bool) {
        self.promotion = val;
    }

    pub fn get_promotion(&self) -> bool {
        self.promotion
    }

    pub fn read_board(&self, file_name: &str) -> Vec<char> {
        let contents = fs::read_to_string(file_name).expect("Could not read the file");

        let res: Vec<char> = contents
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        res
    }

    pub fn fill_board(&mut self, file_name: &str) {
        let start_board = self.read_board(file_name);

        for i in 0..8 {
            for j in 0..8 {
                self.grid[i][j].piece.variety = get_variety(start_board[i * 8 + j]);

                if !self.grid[i][j].is_empty() {
                    self.grid[i][j].piece.color = get_color(start_board[i * 8 + j]);
                }
            }
        }
    }

    pub fn get_moves(&self, pos: &str) -> Vec<(i64, i64)> {
        let (x, y) = string_to_position(pos);
        if x <= 7 && y <= 7 {
            let piece = self.grid[y][x].piece;
            if piece.color.forward() == self.current_player.forward() {
                return piece.variety.get_moves(x, y, &self);
            }
        }
        Vec::<(i64, i64)>::new()
    }

    //Returns game_over, valid_move and (error) message
    pub fn make_move(&mut self, input: &str) -> (bool, bool, String) {
        let tokens: Vec<&str> = input.split("=").collect();
        let pos = tokens[0];
        let mut promotion = "".to_string();
        if tokens.len() > 1 {
            promotion.push_str(tokens[1]);
            self.promotion = true;
            println!("Promotion is true");
        } else {
            self.promotion = false;
        }

        if let "O-O-O" = pos {
            let (valid, king_org_x, king_new_x, rook_org_x, rook_new_x, y) =
                moves::queenside_castling(self);
            if valid {
                self.grid[y][king_new_x] = self.grid[y][king_org_x];
                self.grid[y][king_org_x] = Square::get_empty();

                self.grid[y][rook_new_x] = self.grid[y][rook_org_x];
                self.grid[y][rook_org_x] = Square::get_empty();
            } else {
                return (false, false, "Queenside castling not possible".to_string());
            }
        } else if let "O-O" = pos {
            let (valid, king_org_x, king_new_x, rook_org_x, rook_new_x, y) =
                moves::kingside_castling(self);
            if valid {
                self.grid[y][king_new_x] = self.grid[y][king_org_x];
                self.grid[y][king_org_x] = Square::get_empty();

                self.grid[y][rook_new_x] = self.grid[y][rook_org_x];
                self.grid[y][rook_org_x] = Square::get_empty();
            } else {
                return (false, false, "Kingside castling not possible".to_string());
            }
        } else {
            let (valid, message) = self.check_if_legal_move(pos, true);
            if !valid {
                return (false, false, message);
            }
            if !promotion.is_empty() {
                self.promotion(get_variety(promotion.chars().next().unwrap()));
            }
        }
        self.current_player = self.current_player.inverse();
        self.print_board();

        let state = self.get_state();
        if !state.0 {
            return (true, true, state.1);
        }
        (false, true, state.1)
    }

    pub fn promotion(&mut self, variety: units::Variety) {
        for i in 0..8 {
            if let units::Variety::Pawn = self.grid[0][i].piece.variety {
                self.grid[0][i].piece.variety = variety;
            }
            if let units::Variety::Pawn = self.grid[7][i].piece.variety {
                self.grid[7][i].piece.variety = variety;
            }
        }
    }

    pub fn get_state(&mut self) -> (bool, String) {
        if self.checkmate() {
            if self.check() {
                return (false, "Checkmate".to_string());
            } else {
                return (false, "Stalemate".to_string());
            }
        } else {
            if self.check() {
                return (true, "Check".to_string());
            } else {
                return (true, "".to_string());
            }
        }
    }

    pub fn check_if_legal_move(&mut self, pos: &str, move_piece: bool) -> (bool, String) {
        let split = pos.split(" ");
        let vec: Vec<&str> = split.collect();

        if vec.len() < 2 {
            return (false, "Invalid move".to_string());
        }

        let (x0, y0) = string_to_position(vec[0]);
        let (x1, y1) = string_to_position(vec[1]);
        let moves = self.get_moves(vec[0]);

        for position in moves.iter() {
            let x = position.0 as i64;
            let y = position.1 as i64;
            if x == (x1 as i64) && y == (y1 as i64) {
                let old_square = self.grid[y1][x1];
                self.grid[y1][x1] = self.grid[y0][x0];
                self.grid[y0][x0] = Square::get_empty();

                if self.check() {
                    self.grid[y0][x0] = self.grid[y1][x1];
                    self.grid[y1][x1] = old_square;
                    return (false, "Check!".to_string());
                } else {
                    if !move_piece {
                        self.grid[y0][x0] = self.grid[y1][x1];
                        self.grid[y1][x1] = old_square;
                    } else {
                        let mut set_passant = false;
                        if let units::Variety::Pawn = self.grid[y1][x1].piece.variety {
                            if ((y1 as i64) - (y0 as i64)).abs() > 1 {
                                set_passant = true;
                            }
                            if old_square.is_empty()
                                && ((y1 as i64) - (y0 as i64)).abs() == 1
                                && ((x1 as i64) - (x0 as i64)).abs() == 1
                            {
                                self.grid[y0][x1] = Square::get_empty();
                            }
                        }

                        if set_passant {
                            self.passant = (x1 as i64, y1 as i64);
                        } else {
                            self.passant = (-1, -1);
                        }
                        self.grid[y1][x1].piece.has_moved = true;
                    }
                    return (true, "".to_string());
                }
            }
        }
        (false, "Invalid move".to_string())
    }

    pub fn get_all_valid_moves(&mut self, positions: Vec<String>) -> Vec<String> {
        let mut valid_moves: Vec<String> = Vec::new();
        for pos1 in positions.iter() {
            for pos2 in positions.iter() {
                let pos = pos1.to_owned() + &" ".to_string() + &pos2.to_owned();
                let (valid, _) = self.check_if_legal_move(&pos, false);
                if valid {
                    valid_moves.push(pos);
                }
            }
        }
        valid_moves
    }

    pub fn checkmate(&mut self) -> bool {
        let mut positions: Vec<String> = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                let pos = position_to_string(i as u8, j as u8);
                positions.push(pos.to_owned());
            }
        }

        let valid_moves = self.get_all_valid_moves(positions);

        if valid_moves.len() > 0 {
            return false;
        }

        true
    }

    pub fn move_check(&mut self, x_org: usize, y_org: usize, x_new: usize, y_new: usize) -> bool {
        let check;

        if x_org != x_new || y_org != y_new {
            let old_square = self.grid[y_new][x_new];
            self.grid[y_new][x_new] = self.grid[y_org][x_org];
            self.grid[y_org][x_org] = Square::get_empty();

            check = self.check();

            self.grid[y_org][x_org] = self.grid[y_new][x_new];
            self.grid[y_new][x_new] = old_square;
        } else {
            check = self.check();
        }

        check
    }

    pub fn check(&self) -> bool {
        let mut king_x = 0;
        let mut king_y = 0;
        for i in 0..8 {
            for j in 0..8 {
                if self.grid[i][j].piece.color.forward() == self.current_player.forward() {
                    match self.grid[i][j].piece.variety {
                        units::Variety::King => {
                            king_x = j;
                            king_y = i;
                        }
                        _ => (),
                    }
                }
            }
        }

        for i in 0..8 {
            for j in 0..8 {
                if !self.grid[i][j].is_empty()
                    && (self.grid[i][j].piece.color.forward() != self.current_player.forward())
                {
                    let moves = self.grid[i][j].piece.variety.get_moves(j, i, &self);
                    for position in moves.iter() {
                        if position.0 == (king_x as i64) && position.1 == (king_y as i64) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn print_board(&self) {
        println!("   a b c d e f g h");
        println!("   ---------------");
        for i in 0..8 {
            print!("{}| ", 8 - i);
            for j in 0..8 {
                let mut square;
                match self.grid[i][j].piece.variety {
                    units::Variety::Pawn => square = "p".to_string(),
                    units::Variety::Bishop => square = "b".to_string(),
                    units::Variety::Knight => square = "n".to_string(),
                    units::Variety::Rook => square = "r".to_string(),
                    units::Variety::Queen => square = "q".to_string(),
                    units::Variety::King => square = "k".to_string(),
                    _ => square = "x".to_string(),
                }
                match self.grid[i][j].piece.color {
                    units::Color::Black => square = square.to_uppercase(),
                    _ => (),
                }
                print!("{} ", square);
            }
            println!("|{}", 8 - i);
        }
        println!("   ---------------");
        println!("   a b c d e f g h");
    }
}
