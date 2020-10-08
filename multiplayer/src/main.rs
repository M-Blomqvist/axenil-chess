use std::env;
use std::io::{prelude::*, Error, ErrorKind, Result};
use std::net::{SocketAddrV4, TcpListener, TcpStream};

pub mod chess_communicator;
pub mod message;
use message::Message;
fn main() {
    let args: Vec<String> = env::args().collect();
    let is_host = match args.get(1).expect("2 Arguments required!").as_str() {
        "host" => true,
        "connect" => false,
        _ => panic!(
            "Pass either 'host' or 'connect' argument with ip to play multiplayer. Got: {}",
            args.get(1).expect("2 Arguments required!").as_str()
        ),
    };
    let ip = args.get(2).expect("Need to provide IPV4 and socket addr");
    let ip: SocketAddrV4 = ip.parse().expect("failed to parse ipv4 and socket addr");
    if is_host {
        start_host(ip);
    } else {
        connect_client(ip).expect("");
    }
}

fn start_host(ip: SocketAddrV4) {
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
        println!("Recieved Message: {}", Message::from(buffer[0]));
        if buffer.contains(&(Message::Accept as u8)) {
            response = [Message::Accept as u8];
        }
    } else {
        println!("Error reading stream")
    }

    stream.write_all(&response).unwrap();
    stream.flush().unwrap();
}

fn connect_client(ip: SocketAddrV4) -> Result<String> {
    if let Ok(mut stream) = TcpStream::connect(ip) {
        println!("Connection established to: {}", ip);
        println!("{}", send_message(&mut stream, Message::Accept)?);
    } else {
        panic!("Error connecting to {}", ip);
    }
    Ok("Connection successfully used & terminated".to_string())
}

pub fn send_message(stream: &mut TcpStream, message: Message) -> Result<String> {
    let mut buffer = [255; 5];
    let message_string = message.to_string();
    stream.write_all(&[message as u8])?;
    println!("Sent {}...", message_string);
    if let Ok(len) = stream.read(&mut buffer) {
        if len <= buffer.len() {
            if buffer.contains(&(Message::Accept as u8)) {
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
