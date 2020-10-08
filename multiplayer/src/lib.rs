use std::io::{prelude::*, Error, ErrorKind, Result};
use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub mod chess_communicator;
pub mod message;
use message::{Message, MoveMessage};

pub fn start_multiplayer(connect_type: &str, ip: &str) -> Result<Sender<[u8; 5]>> {
    let is_host = match connect_type {
        "host" => true,
        "connect" => false,
        _ => panic!(
            "Pass either 'host' or 'connect' argument with ip to play multiplayer. Got: {}",
            connect_type
        ),
    };
    let ip: SocketAddrV4 = ip.parse().expect("failed to parse ipv4 and socket addr");
    if is_host {
        start_host(ip)
    } else {
        connect_client(ip)
    }
}

fn start_host(ip: SocketAddrV4) -> Result<Sender<[u8; 5]>> {
    let listener = TcpListener::bind(ip).unwrap();
    println!("Started host at {}", ip);

    if let Ok((mut stream, client)) = listener.accept() {
        println!("Connection from {}", client);

        let buffer = [255; 5];
        recieve_message(&mut stream, buffer, Some(Message::Accept))?;
        println!("{}", send_message(&mut stream, Message::Accept)?);

        let (sender, reciever) = channel::<[u8; 5]>();
        thread::spawn(move || {
            std_loop(&mut stream, reciever);
        });
        Ok(sender)
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "Error setting up host & sender".to_string(),
        ))
    }
}

fn connect_client(ip: SocketAddrV4) -> Result<Sender<[u8; 5]>> {
    if let Ok(mut stream) = TcpStream::connect(ip) {
        println!("Connection established to: {}", ip);

        println!("{}", send_message(&mut stream, Message::Accept)?);

        let buffer = [255; 5];
        recieve_message(&mut stream, buffer, Some(Message::Accept))?;

        let (sender, reciever) = channel::<[u8; 5]>();
        thread::spawn(move || {
            std_loop(&mut stream, reciever);
        });
        Ok(sender)
    } else {
        panic!("Error connecting to {}", ip);
    }
}

fn std_loop(stream: &mut TcpStream, rx: Receiver<[u8; 5]>) {
    let buffer = [255; 5];
    loop {
        if let Ok(message) = rx.try_recv() {
            let message_type = Message::from(message);
            if message_type != Message::Move {
                if let Err(error) = send_message(stream, message_type) {
                    println!("Error sending message: {}", error.to_string());
                }
            }
        }
        recieve_message(stream, buffer, None).expect("error reading message");
        thread::sleep(Duration::from_millis(1));
    }
}

fn send_message(stream: &mut TcpStream, message: Message) -> Result<String> {
    let message_string = message.to_string();
    stream.write_all(&[message as u8])?;
    stream.flush()?;
    Ok(format!("Sent {}...", message_string))
}

// fn send_move(stream: &mut TcpStream, message: MoveMessage) -> Result<String> {
//     let message_string = message.to_string();
//     stream.write_all(&message)?;
//     stream.flush()?;
//     Ok(format!("Sent {}...", message_string))
// }

fn recieve_message(
    stream: &mut TcpStream,
    mut buffer: [u8; 5],
    expect_message: Option<Message>,
) -> Result<[u8; 5]> {
    if let Ok(len) = stream.read(&mut buffer) {
        if buffer == [255; 5] {
            return Ok(buffer);
        }
        if len <= buffer.len() {
            if let Some(mess) = expect_message {
                if mess == buffer[0] {
                    println!("Got expected {}!", mess);
                    Ok(buffer)
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
                println!("Recieved {}!", Message::from(buffer[0]));
                Ok(buffer)
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
