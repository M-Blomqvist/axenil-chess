#[derive(Copy, Clone, PartialEq)]
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

pub enum MoveMessage {
    Standard(),
    En_Passant(),
    Promotion(),
    Kingside_castle(),
    Queenside_castle(),
}

impl PartialEq<Message> for MoveMessage {
    fn eq(&self, other: &Message) -> bool {
        *other == Message::Move
    }
}

impl PartialEq<u8> for Message {
    fn eq(&self, other: &u8) -> bool {
        *self as u8 == *other
    }
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

impl From<[u8; 5]> for Message {
    fn from(bytes: [u8; 5]) -> Self {
        match bytes[0] {
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
