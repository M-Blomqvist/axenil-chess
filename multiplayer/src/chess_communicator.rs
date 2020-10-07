pub enum Messages {
    Decline = 0x00,
    Move = 0x01,
    Undo = 0x02,
    Accept = 0x03,
    Checkmate = 0x04,
    Draw = 0x05,
    Resign = 0x06,
}

// fn send_move(move_result: (bool, bool, String)) -> u8 {
//     if move_result.0 {}
// }
