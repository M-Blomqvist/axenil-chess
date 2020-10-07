use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::net::{TcpListener, TcpStream};

mod chess_communicator;
use chess_communicator::Messages;

fn main() {
    let ip = "192.168.1.77:7878";
    start_host(ip);
}

fn start_host(ip: &str) {
    let listener = TcpListener::bind(ip).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

#[allow(clippy::unused_io_amount)]
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [255; 5];

    stream.read(&mut buffer).unwrap();

    let response = if buffer.contains(&(Messages::Accept as u8)) {
        [Messages::Accept as u8]
    } else {
        [Messages::Decline as u8]
    };

    stream.write_all(&response).unwrap();
    stream.flush().unwrap();
}
fn connect_client(ip: &str) -> Result<String> {
    if let Ok(mut stream) = TcpStream::connect(ip) {
        println!("Connection established to: {}", ip);
        stream.write_all(&[Messages::Accept as u8])?;
        println!("Sent accept message...");
        let mut buffer = [255; 5];
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
    } else {
        panic!("Error connecting to {}", ip);
    }
}
