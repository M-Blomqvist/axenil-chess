use std::io::Result;
use std::net::TcpStream;

use multiplayer::{chess_communicator::Message, send_message};
fn main() {
    let ip = "192.168.1.77:7878";
    println!("{}", connect_client(ip).unwrap());
}

fn connect_client(ip: &str) -> Result<String> {
    if let Ok(mut stream) = TcpStream::connect(ip) {
        println!("Connection established to: {}", ip);
        println!("{}", send_message(&mut stream, Message::Accept)?);
    } else {
        panic!("Error connecting to {}", ip);
    }
    Ok("Connection successfully used & terminated".to_string())
}
