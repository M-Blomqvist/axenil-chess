use std::net::{SocketAddrV4, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;
use std::{
    io::{prelude::*, Error, ErrorKind, Result},
    thread::JoinHandle,
};

pub mod chess_communicator;
pub mod message;
use message::{Message, MoveMessage};

pub type OnlineConnection<T> = (Sender<T>, Receiver<T>);

pub fn start_multiplayer(
    connect_type: &str,
    ip: &str,
) -> Result<(OnlineConnection<[u8; 5]>, JoinHandle<()>)> {
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

fn start_host(ip: SocketAddrV4) -> Result<(OnlineConnection<[u8; 5]>, JoinHandle<()>)> {
    let listener = TcpListener::bind(ip).unwrap();
    println!("Started host at {}", ip);
    if let Ok((mut stream, client)) = listener.accept() {
        println!("Connection from {}", client);
        stream.set_nonblocking(true)?;

        let mut buffer = [255; 5];
        loop {
            let result = recieve_message(&mut stream, buffer, Some(Message::Accept));
            if result.is_ok() {
                break;
            } else {
                let err = result.unwrap_err();
                if err.kind() != ErrorKind::WouldBlock {
                    return Err(err);
                }
            }
        }
        send_message(&mut stream, Message::Accept)?;

        let (out_sender, out_reciever) = channel::<[u8; 5]>();
        let (in_sender, in_reciever) = channel::<[u8; 5]>();
        let handle = thread::spawn(move || {
            std_loop(stream, (in_sender, out_reciever));
        });
        Ok(((out_sender, in_reciever), handle))
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "Error setting up host & sender".to_string(),
        ))
    }
}

fn connect_client(ip: SocketAddrV4) -> Result<(OnlineConnection<[u8; 5]>, JoinHandle<()>)> {
    if let Ok(mut stream) = TcpStream::connect(ip) {
        println!("Connection established to: {}", ip);
        stream.set_nonblocking(true)?;

        send_message(&mut stream, Message::Accept)?;

        let buffer = [255; 5];
        loop {
            let result = recieve_message(&mut stream, buffer, Some(Message::Accept));
            if result.is_ok() {
                break;
            } else {
                let err = result.unwrap_err();
                if err.kind() != ErrorKind::WouldBlock {
                    return Err(err);
                }
            }
        }

        let (out_sender, out_reciever) = channel::<[u8; 5]>();
        let (in_sender, in_reciever) = channel::<[u8; 5]>();
        let handle = thread::spawn(move || {
            std_loop(stream, (in_sender, out_reciever));
        });
        Ok(((out_sender, in_reciever), handle))
    } else {
        panic!("Error connecting to {}", ip);
    }
}

fn std_loop(mut stream: TcpStream, connection: OnlineConnection<[u8; 5]>) {
    loop {
        let buffer = [255; 5];
        if let Ok(message) = connection.1.try_recv() {
            let message_type = Message::from(message);
            if message_type != Message::Move {
                if let Err(error) = send_message(&mut stream, message_type) {
                    println!("Error sending message: {}", error.to_string());
                }
            } else if let Err(error) = send_move(&mut stream, message) {
                println!("Error sending message: {}", error.to_string());
            }
        } else {
            let result = recieve_message(&mut stream, buffer, None);
            if let Ok(message) = result {
                if message != [255; 5] {
                    if Message::Move != message[0] {
                        if let Err(error) = send_message(&mut stream, Message::Accept) {
                            println!("Error sending message: {}", error.to_string());
                        }
                    }
                    connection
                        .0
                        .send(message)
                        .expect("Error sending message from reciever thread");
                }
            } else {
                let err = result.unwrap_err();
                if err.kind() != ErrorKind::WouldBlock {
                    panic!(err.to_string());
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn send_message(stream: &mut TcpStream, message: Message) -> Result<()> {
    let message_string = message.to_string();
    let s = stream.write(&[message as u8])?;
    stream.flush()?;
    println!("Sent {}...", message_string);
    Ok(())
}

fn send_move(stream: &mut TcpStream, message: [u8; 5]) -> Result<String> {
    let message_string = message[0].to_string();
    let s = stream.write(&message)?;
    stream.flush()?;
    Ok(format!("Sent {}...", message_string))
}

fn recieve_message(
    stream: &mut TcpStream,
    mut buffer: [u8; 5],
    expect_message: Option<Message>,
) -> Result<[u8; 5]> {
    let result = stream.read(&mut buffer[..]);
    if result.is_ok() {
        if buffer == [255; 5] {
            return Ok(buffer);
        }
        if result.unwrap() <= buffer.len() {
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
        Err(result.unwrap_err())
    }
}
