#![allow(clippy::all)]
use rust_chess::board;
use rust_chess::units;

#[test]
fn create_square() {
    let square = board::Square::get_empty();
    assert!(square.is_empty());
}

#[test]
fn string_to_pos_test() {
    let (x, y) = board::string_to_position("e1");
    assert_eq!((x, y), (4, 7));
    let (x1, y1) = board::string_to_position("g5");
    assert_eq!((x1, y1), (6, 3));
}

#[test]
fn get_color_test() {
    let white = board::get_color('r');
    match white {
        units::Color::White => assert!(true),
        _ => assert!(false),
    }

    let black = board::get_color('R');
    match black {
        units::Color::Black => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn initiate_board() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./data/board.txt");

    let square = chess_board.get_square(0, 0);

    assert!(!square.is_empty());
}

#[test]
fn get_state_test() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./data/board.txt");

    assert!(chess_board.get_state().0);
}

#[test]
fn gibberish() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./data/board.txt");

    chess_board.make_move("abcadsak");
    chess_board.make_move("ab ab ab");
    chess_board.make_move("e2 e9");
}
