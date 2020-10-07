use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::net::{TcpListener, TcpStream};

pub mod chess_communicator;
use chess_communicator::Messages;

pub fn send_message(stream: &mut TcpStream, message: Messages) -> Result<String> {
    let mut buffer = [255; 5];
    let message_string = message.to_string();
    stream.write_all(&[message as u8])?;
    println!("Sent {}...", message_string);
    if let Ok(len) = stream.read(&mut buffer) {
        if len <= buffer.len() {
            if buffer.contains(&(Messages::Accept as u8)) {
                Ok(format!(
                    "Got accept message back! Message: {}",
                    String::from_utf8_lossy(&buffer[..])
                ))
            } else {
                Err(Error::new(
                    ErrorKind::Other,
                    "No accept message back!".to_string(),
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::Other,
                "Buffer shorter than message!".to_string(),
            ))
        }
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "Error reading response from stream!".to_string(),
        ))
    }
}
