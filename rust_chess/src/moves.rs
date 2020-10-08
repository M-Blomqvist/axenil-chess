#![allow(clippy::all)]
use crate::board;
use crate::units;
use std::cmp;

pub fn get_possible_moves(
    x_org: usize,
    y_org: usize,
    chess_piece: units::Piece,
    chess_board: &board::Board,
    capture: bool,
) -> Vec<(i64, i64)> {
    let mut move_vec: Vec<(i64, i64, bool)> = chess_piece.get_moves();
    let mut moves = Vec::<(i64, i64)>::new();
    while let Some((x, y, mut multiple_steps)) = move_vec.pop() {
        if y + (y_org as i64) < 0
            || x + (x_org as i64) < 0
            || y + (y_org as i64) > 7
            || x + (x_org as i64) > 7
        {
            continue;
        }
        let y_index = (y + (y_org as i64)) as usize;
        let x_index = (x + (x_org as i64)) as usize;
        let next_square = chess_board.get_square(x_index, y_index);

        if !next_square.is_empty() {
            if (chess_piece.color.forward() == next_square.piece.color.forward()) || !capture {
                continue;
            } else {
                multiple_steps = false;
            }
        }

        moves.push((x + (x_org as i64), y + (y_org as i64)));
        if multiple_steps {
            let mut next_x = 0;
            let mut next_y = 0;
            if x < 0 {
                next_x = x - 1;
            }
            if x > 0 {
                next_x = x + 1;
            }
            if y < 0 {
                next_y = y - 1;
            }
            if y > 0 {
                next_y = y + 1;
            }
            move_vec.push((next_x, next_y, multiple_steps))
        }
    }
    moves
}

pub fn get_pawn_capture_moves(
    x_org: usize,
    y_org: usize,
    chess_piece: units::Piece,
    chess_board: &board::Board,
) -> Vec<(i64, i64)> {
    let mut moves = Vec::<(i64, i64)>::new();
    if y_org > 6 || y_org < 1 {
        return moves;
    }
    let y = ((y_org as i64) + chess_piece.color.forward()) as usize;

    if x_org + 1 < 8 {
        let next_square = chess_board.get_square(x_org + 1, y);
        if (next_square.piece.color.forward() != chess_piece.color.forward())
            && !next_square.is_empty()
        {
            moves.push(((x_org + 1) as i64, y as i64));
        }
    }

    if (x_org as i64) - 1 >= 0 {
        let next_square = chess_board.get_square(x_org - 1, y);
        if (next_square.piece.color.forward() != chess_piece.color.forward())
            && !next_square.is_empty()
        {
            moves.push(((x_org - 1) as i64, y as i64));
        }
    }

    moves
}

pub fn pawn_two_steps(
    x_org: usize,
    y_org: usize,
    chess_piece: units::Piece,
    chess_board: &board::Board,
) -> Vec<(i64, i64)> {
    let mut moves = Vec::<(i64, i64)>::new();
    let y1 = (y_org as i64) + chess_piece.color.forward();
    let y2 = (y_org as i64) + chess_piece.color.forward() * 2;
    if !chess_piece.has_moved && y2 <= 7 && y2 >= 0 {
        let one_forward = chess_board.get_square(x_org, y1 as usize);
        let two_forward = chess_board.get_square(x_org, y2 as usize);
        if one_forward.is_empty() && two_forward.is_empty() {
            moves.push((x_org as i64, y2));
        }
    }
    moves
}

pub fn pawn_passant(
    x_org: usize,
    y_org: usize,
    chess_piece: units::Piece,
    chess_board: &board::Board,
) -> Vec<(i64, i64)> {
    let mut moves = Vec::<(i64, i64)>::new();
    let passant = chess_board.get_passant();
    if passant.1 == (y_org as i64) && ((passant.0 as i64) - (x_org as i64)).abs() == 1 {
        let y = (y_org as i64) + chess_piece.color.forward();
        let x = passant.0 as i64;
        if y >= 0 && y <= 7 && x >= 0 && x <= 7 {
            if chess_board.get_square(x as usize, y as usize).is_empty() {
                moves.push((x, y));
            }
        }
    }
    moves
}

pub fn move_pawn(x: usize, y: usize, chess_board: &board::Board) -> Vec<(i64, i64)> {
    let mut moves = Vec::<(i64, i64)>::new();
    let chess_piece = chess_board.get_square(x, y).piece;

    let normal_moves = move_normal(x, y, chess_board, false);
    let capture_moves = get_pawn_capture_moves(x, y, chess_piece, chess_board);
    let two_steps = pawn_two_steps(x, y, chess_piece, chess_board);
    let passant_moves = pawn_passant(x, y, chess_piece, chess_board);

    for normal_move in normal_moves.iter() {
        moves.push(normal_move.to_owned());
    }

    for capture_move in capture_moves.iter() {
        moves.push(capture_move.to_owned());
    }

    for two_step_move in two_steps.iter() {
        moves.push(two_step_move.to_owned());
    }

    for passant_move in passant_moves.iter() {
        moves.push(passant_move.to_owned());
    }

    let mut legal_moves = Vec::<(i64, i64)>::new();
    for mv in moves.iter() {
        if !chess_board.get_promotion() {
            if mv.1 < 7 && mv.1 > 0 {
                legal_moves.push(mv.to_owned());
            }
        } else {
            legal_moves.push(mv.to_owned());
        }
    }

    legal_moves
}

pub fn move_normal(
    x: usize,
    y: usize,
    chess_board: &board::Board,
    capture: bool,
) -> Vec<(i64, i64)> {
    let chess_piece = chess_board.get_square(x, y).piece;
    let possible_moves = get_possible_moves(x, y, chess_piece, chess_board, capture);
    possible_moves
}

pub fn castling(
    chess_board: &mut board::Board,
    king_org_x: usize,
    king_new_x: usize,
    rook_org_x: usize,
    rook_new_x: usize,
    y: usize,
) -> (bool, usize, usize, usize, usize, usize) {
    let king = chess_board.get_square(king_org_x, y).piece;
    let rook = chess_board.get_square(rook_org_x, y).piece;
    if let units::Variety::King = king.variety {
        if let units::Variety::Rook = rook.variety {
            if !king.has_moved && !rook.has_moved {
                let mn_row = cmp::min(king_org_x, rook_org_x);
                let mx_row = cmp::max(king_org_x, rook_org_x);

                for x_pos in (mn_row + 1)..(mx_row) {
                    if !chess_board.get_square(x_pos, y).is_empty() {
                        return (false, king_org_x, king_new_x, rook_org_x, rook_new_x, y);
                    }
                }

                let mn = cmp::min(king_org_x, king_new_x);
                let mx = cmp::max(king_org_x, king_new_x);

                for x_pos in mn..(mx + 1) {
                    if chess_board.move_check(king_org_x, y, x_pos, y) {
                        return (false, king_org_x, king_new_x, rook_org_x, rook_new_x, y);
                    }
                }

                return (true, king_org_x, king_new_x, rook_org_x, rook_new_x, y);
            }
        }
    }
    (false, king_org_x, king_new_x, rook_org_x, rook_new_x, y)
}

pub fn kingside_castling(
    chess_board: &mut board::Board,
) -> (bool, usize, usize, usize, usize, usize) {
    let mut y = 0;
    if let units::Color::White = chess_board.get_current_player() {
        y = 7;
    }

    castling(chess_board, 4, 6, 7, 5, y)
}

pub fn queenside_castling(
    chess_board: &mut board::Board,
) -> (bool, usize, usize, usize, usize, usize) {
    let mut y = 0;
    if let units::Color::White = chess_board.get_current_player() {
        y = 7;
    }
    castling(chess_board, 4, 2, 0, 3, y)
}
