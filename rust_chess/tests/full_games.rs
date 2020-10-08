#![allow(clippy::all)]
use rust_chess::board;

#[test]
fn game1() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./data/board.txt");

    assert!(chess_board.make_move("e2 e4").1);
    assert!(chess_board.make_move("e7 e5").1);

    assert!(chess_board.make_move("g1 f3").1);
    assert!(chess_board.make_move("f7 f6").1);

    assert!(chess_board.make_move("f3 e5").1);
    assert!(chess_board.make_move("f6 e5").1);

    assert!(chess_board.make_move("d1 h5").1);
    assert!(chess_board.check());
    assert!(chess_board.make_move("e8 e7").1);

    assert!(chess_board.make_move("h5 e5").1);
    assert!(chess_board.check());
    assert!(chess_board.make_move("e7 f7").1);

    assert!(chess_board.make_move("f1 c4").1);
    assert!(chess_board.check());
    assert!(chess_board.make_move("d7 d5").1);

    assert!(chess_board.make_move("c4 d5").1);
    assert!(chess_board.check());
    assert!(chess_board.make_move("f7 g6").1);

    assert!(chess_board.make_move("h2 h4").1);
    assert!(chess_board.make_move("h7 h5").1);

    assert!(chess_board.make_move("d5 b7").1);
    assert!(chess_board.make_move("c8 b7").1);

    assert!(chess_board.make_move("e5 f5").1);
    assert!(chess_board.check());
    assert!(chess_board.make_move("g6 h6").1);

    assert!(chess_board.make_move("d2 d4").1);
    assert!(chess_board.check());
    assert!(chess_board.make_move("g7 g5").1);

    assert!(chess_board.make_move("f5 f7").1);
    assert!(chess_board.make_move("d8 e7").1);

    assert!(chess_board.make_move("h4 g5").1);
    assert!(chess_board.make_move("e7 g5").1);

    assert!(chess_board.make_move("h1 h5").1);
    if "Checkmate" == chess_board.get_state().1 {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn game2() {
    let mut chess_board = board::Board::init();
    chess_board.fill_board("./data/board.txt");

    assert!(chess_board.make_move("e2 e4").1);
    assert!(chess_board.make_move("e7 e5").1);

    assert!(chess_board.make_move("g1 f3").1);
    assert!(chess_board.make_move("b8 c6").1);

    assert!(chess_board.make_move("f1 c4").1);
    assert!(chess_board.make_move("f8 c5").1);

    assert!(chess_board.make_move("c2 c3").1);
    assert!(chess_board.make_move("g8 f6").1);

    assert!(chess_board.make_move("d2 d4").1);
    assert!(chess_board.make_move("e5 d4").1);

    assert!(chess_board.make_move("e4 e5").1);
    assert!(chess_board.make_move("f6 e4").1);

    assert!(chess_board.make_move("c4 d5").1);
    assert!(chess_board.make_move("e4 f2").1);

    assert!(chess_board.make_move("e1 f2").1);
    assert!(chess_board.make_move("d4 c3").1);
    assert!(chess_board.check());

    assert!(chess_board.make_move("f2 g3").1);
    assert!(chess_board.make_move("c3 b2").1);

    assert!(chess_board.make_move("c1 b2").1);
    assert!(chess_board.make_move("c6 e7").1);

    assert!(chess_board.make_move("f3 g5").1);
    assert!(chess_board.make_move("e7 d5").1);

    assert!(chess_board.make_move("g5 f7").1);
    assert!(chess_board.make_move("O-O").1);

    assert!(chess_board.make_move("f7 d8").1);
    assert!(chess_board.make_move("c5 f2").1);
    assert!(chess_board.check());

    assert!(chess_board.make_move("g3 h3").1);
    assert!(chess_board.make_move("d7 d6").1);
    assert!(chess_board.check());

    assert!(chess_board.make_move("e5 e6").1);
    assert!(chess_board.make_move("d5 f4").1);
    assert!(chess_board.check());

    assert!(chess_board.make_move("h3 g4").1);
    assert!(chess_board.make_move("f4 e6").1);

    assert!(chess_board.make_move("d8 e6").1);
    assert!(chess_board.make_move("c8 e6").1);
    assert!(chess_board.check());

    assert!(chess_board.make_move("g4 g5").1);
    assert!(chess_board.make_move("f8 f5").1);
    assert!(chess_board.check());

    assert!(chess_board.make_move("g5 g4").1);
    assert!(chess_board.make_move("h7 h5").1);
    assert!(chess_board.check());

    assert!(chess_board.make_move("g4 h3").1);
    assert!(chess_board.make_move("f5 f3").1);

    if "Checkmate" == chess_board.get_state().1 {
        assert!(true);
    } else {
        assert!(false);
    }
}
