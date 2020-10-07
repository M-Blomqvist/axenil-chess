use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use multiplayer::chess_communicator::*;
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

    let mut response = [Message::Decline as u8];

    if let Ok(_) = stream.read(&mut buffer) {
        println!("Recieved Message: {:?}", buffer);
        for part in buffer.iter() {
            println!("{}", Message::from(*part).to_string());
        }
        if buffer.contains(&(Message::Accept as u8)) {
            response = [Message::Accept as u8];
        }
    } else {
        println!("Error reading stream")
    }

    stream.write_all(&response).unwrap();
    stream.flush().unwrap();
}
