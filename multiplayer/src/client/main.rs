use std::io::Result;
use std::net::TcpStream;

use multiplayer::{chess_communicator::Messages, send_message};
fn main() {
    let ip = "192.168.1.77:7878";
    connect_client(ip);
}

fn connect_client(ip: &str) -> Result<String> {
    if let Ok(mut stream) = TcpStream::connect(ip) {
        println!("Connection established to: {}", ip);
        println!("{}", send_message(&mut stream, Messages::Accept)?);
    } else {
        panic!("Error connecting to {}", ip);
    }
    Ok("Connection successfully used & terminated".to_string())
}
