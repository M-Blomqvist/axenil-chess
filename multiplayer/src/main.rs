use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
fn main() {
    let ip = "192.168.1.77:7878";
    start_host(ip);
}

#[allow(clippy::unused_io_amount)]
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let response = if buffer.contains(&(Messages::Accept as u8)) {
        [Messages::Accept as u8]
    } else {
        [Messages::Decline as u8]
    };

    stream.write(&response).unwrap();
    stream.flush().unwrap();
}

fn start_host(ip: &str) {
    let listener = TcpListener::bind(ip).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
#[allow(clippy::unused_io_amount)]
fn connect_client(ip: &str) {
    if let Ok(mut stream) = TcpStream::connect(ip) {
        println!("Connection established to: {}", ip);
        stream.write(&[Messages::Accept as u8]).unwrap();
        println!("Sent accept message...");
        let mut buffer = [0; 1024];
        if let Ok(_) = stream.read(&mut buffer) {
            if buffer.contains(&(Messages::Accept as u8)) {
                println!("Got accept message back!");
            } else {
                println!("No accept message back!");
            }
            println!("{}", String::from_utf8_lossy(&buffer[..]));
        } else {
            println!("Error reading response from stream!");
        }
    } else {
        panic!("Error connecting to {}", ip);
    }
}

#[repr(u8)]
enum Messages {
    Decline = 0x00,
    Move = 0x01,
    Undo = 0x02,
    Accept = 0x03,
    Checkmate = 0x04,
    Draw = 0x05,
    Resign = 0x06,
}

// impl Messages {
//     fn get_move_message(mov: (u8, u8)) -> (u8, u8) {}
// }
