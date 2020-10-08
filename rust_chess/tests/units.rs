#![allow(clippy::all)]
use rust_chess::units;

#[test]
fn check_color() {
    let c1 = units::Color::Black;

    match c1.inverse() {
        units::Color::White => assert!(true),
        _ => assert!(false),
    }

    assert_eq!(c1.forward(), 1);
}
