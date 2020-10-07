pub enum Message {
    Decline = 0x00,
    Move = 0x01,
    Undo = 0x02,
    Accept = 0x03,
    Checkmate = 0x04,
    Draw = 0x05,
    Resign = 0x06,
    Unknown = 254,
}

impl From<u8> for Message {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Message::Decline,
            0x01 => Message::Move,
            0x02 => Message::Undo,
            0x03 => Message::Accept,
            0x04 => Message::Checkmate,
            0x06 => Message::Draw,
            0x05 => Message::Resign,
            _ => Message::Unknown,
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message_str = match self {
            Message::Decline => "Decline message",
            Message::Move => "Move message",
            Message::Undo => "Undo message",
            Message::Accept => "Accept message",
            Message::Checkmate => "Checkmate message",
            Message::Draw => "Draw message",
            Message::Resign => "Resign message",
            _ => "Unknown message",
        };
        write!(f, "{}", message_str)
    }
}
