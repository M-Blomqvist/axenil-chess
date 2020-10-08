#![allow(clippy::all)]
use rust_chess::board;
//use rust_chess::units;

#[test]
fn test_legal_moves() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./data/board.txt");

    //Legal moves
    assert!(chess_board.check_if_legal_move("a2 a4", false).0);
    assert!(chess_board.check_if_legal_move("b2 b4", false).0);
    assert!(chess_board.check_if_legal_move("c2 c4", false).0);
    assert!(chess_board.check_if_legal_move("d2 d3", false).0);
    assert!(chess_board.check_if_legal_move("e2 e4", false).0);
    assert!(chess_board.check_if_legal_move("f2 f3", false).0);
    assert!(chess_board.check_if_legal_move("g2 g4", false).0);
    assert!(chess_board.check_if_legal_move("h2 h3", false).0);

    assert!(chess_board.check_if_legal_move("b1 c3", false).0);
    assert!(chess_board.check_if_legal_move("b1 a3", false).0);
    assert!(chess_board.check_if_legal_move("g1 h3", false).0);
    assert!(chess_board.check_if_legal_move("g1 f3", false).0);

    //Illegal moves
    assert!(!chess_board.check_if_legal_move("a7 a6", false).0);
    assert!(!chess_board.check_if_legal_move("b8 c6", false).0);
    assert!(!chess_board.check_if_legal_move("c1 e3", false).0);
    assert!(!chess_board.check_if_legal_move("b4 f4", false).0);
}

#[test]
fn test_random_moves() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./tests/test_data/random.txt");

    assert!(!chess_board.make_move("h1 h4").1);
    assert!(chess_board.make_move("h1 h3").1);
    assert!(!chess_board.make_move("d8 d6").1);
    assert!(chess_board.make_move("d8 d7").1);
    assert!(chess_board.make_move("b1 a3").1);
    assert!(chess_board.make_move("a7 a5").1);
    assert!(chess_board.make_move("e1 d1").1);
}

#[test]
fn test_queenside_castling() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./tests/test_data/castling.txt");

    assert!(chess_board.make_move("O-O-O").1);
    assert!(!chess_board.make_move("O-O-O").1);
}

#[test]
fn test_kingside_castling() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./tests/test_data/castling.txt");

    assert!(chess_board.make_move("O-O").1);
    assert!(!chess_board.make_move("O-O").1);
}

#[test]
fn castling_after_moved() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./tests/test_data/castling.txt");

    chess_board.make_move("e1 e2");
    chess_board.make_move("e8 e7");
    chess_board.make_move("e2 e1");
    chess_board.make_move("e7 e8");
    assert!(!chess_board.make_move("O-O-O").1);
    assert!(!chess_board.make_move("O-O-O").1);
}

#[test]
fn stalemate_test() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./tests/test_data/stalemate.txt");

    chess_board.make_move("h3 h2");
    if "Stalemate" == chess_board.get_state().1 {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn checkmate_test() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./tests/test_data/checkmate.txt");

    chess_board.make_move("h2 h1");

    if "Checkmate" == chess_board.get_state().1 {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn passant_test() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./tests/test_data/passant.txt");

    chess_board.make_move("c4 c5");
    chess_board.make_move("b7 b5");
    assert!(chess_board.make_move("c5 b6").1);
    assert!(chess_board.get_square(1, 3).is_empty());
}

#[test]
fn promotion_test() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./tests/test_data/promotion.txt");

    assert!(!chess_board.make_move("c7 c8").1);
    assert!(chess_board.make_move("c7 c8=Q").1);
}

#[test]
fn promotion_capture_test() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./tests/test_data/promotion.txt");

    assert!(chess_board.make_move("c7 d8=Q").1);
}

#[test]
fn black_promotion() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./tests/test_data/black_promotion.txt");

    assert!(chess_board.make_move("d2 d1").1);
    assert!(!chess_board.make_move("c2 c1").1);
    assert!(chess_board.make_move("c2 d1=Q").1);
}
