pub enum Messages {
    Decline = 0x00,
    Move = 0x01,
    Undo = 0x02,
    Accept = 0x03,
    Checkmate = 0x04,
    Draw = 0x05,
    Resign = 0x06,
}

pub fn byte_to_string(message: &u8) -> String {
    match message {
        Decline => "Decline message".to_string(),
        Move => "Move message".to_string(),
        Undo => "Undo message".to_string(),
        Accept => "Accept message".to_string(),
        Checkmate => "Checkmate message".to_string(),
        Draw => "Draw message".to_string(),
        Resign => "Resign message".to_string(),
        255 => "".to_string(),
        _ => "Unknown message".to_string(),
    }
}
