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

    let mut response = if recieve_message(&mut stream, &mut buffer, Some(Message::Accept)).is_ok() {
        [Message::Accept as u8]
    } else {
        [Message::Decline as u8]
    };

    stream.write_all(&response).unwrap();
    stream.flush().unwrap();
}

fn connect_client(ip: SocketAddrV4) -> Result<String> {
    if let Ok(mut stream) = TcpStream::connect(ip) {
        println!("Connection established to: {}", ip);
        println!("{}", send_message(&mut stream, Message::Accept)?);
        let mut buffer = [255; 5];
        println!(
            "{}",
            recieve_message(&mut stream, &mut buffer, Some(Message::Accept))?
        );
    } else {
        panic!("Error connecting to {}", ip);
    }
    Ok("Connection successfully used & terminated".to_string())
}

fn send_message(stream: &mut TcpStream, message: Message) -> Result<String> {
    let message_string = message.to_string();
    stream.write_all(&[message as u8])?;
    Ok(format!("Sent {}...", message_string))
}

fn recieve_message(
    stream: &mut TcpStream,
    buffer: &mut [u8; 5],
    expect_message: Option<Message>,
) -> Result<String> {
    if let Ok(len) = stream.read(buffer) {
        if len <= buffer.len() {
            if let Some(mess) = expect_message {
                if buffer.contains(&(mess as u8)) {
                    Ok(format!("Got expected {} back!", mess))
                } else {
                    Err(Error::new(
                        ErrorKind::Other,
                        format!(
                            "Expected message not recieved! Got: {}",
                            Message::from(buffer[0])
                        ),
                    ))
                }
            } else {
                Ok(format!("Recieved {}!", Message::from(buffer[0])))
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
            "Error reading stream!".to_string(),
        ))
    }
}
