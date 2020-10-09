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

pub fn start_multiplayer(connect_type: &str, ip: &str) -> Result<()> {
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

fn start_host(ip: SocketAddrV4) -> Result<()> {
    let listener = TcpListener::bind(ip).unwrap();
    println!("Started host at {}", ip);
    if let Ok((mut stream, client)) = listener.accept() {
        println!("Connection from {}", client);

        let mut buffer = [255; 5];
        recieve_message(&mut stream, buffer, Some(Message::Accept))?;
        send_message(&mut stream, Message::Accept)?;

        println!("{:?}", buffer.iter());
        let (sender, reciever) = channel::<[u8; 5]>();
        let handle = thread::spawn(move || {
            std_loop(stream, reciever, false);
        });

        handle.join().unwrap();
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "Error setting up host & sender".to_string(),
        ))
    }
}

fn connect_client(ip: SocketAddrV4) -> Result<()> {
    if let Ok(mut stream) = TcpStream::connect(ip) {
        println!("Connection established to: {}", ip);

        send_message(&mut stream, Message::Accept)?;

        let buffer = [255; 5];
        recieve_message(&mut stream, buffer, Some(Message::Accept))?;

        let (sender, reciever) = channel::<[u8; 5]>();
        let handle = thread::spawn(move || {
            std_loop(stream, reciever, true);
        });
        for i in 0..6 {
            sender.send([i; 5]).expect("failed in sending message");
            thread::sleep(Duration::from_millis(100))
        }
        handle.join().unwrap();
        Ok(())
    } else {
        panic!("Error connecting to {}", ip);
    }
}

fn std_loop(mut stream: TcpStream, rx: Receiver<[u8; 5]>, mut is_sending: bool) {
    loop {
        let buffer = [255; 5];
        if is_sending {
            let message = [Message::Accept as u8; 5];
            let message_type = Message::from(message);
            if message_type != Message::Move {
                if let Err(error) = send_message(&mut stream, message_type) {
                    println!("Error sending message: {}", error.to_string());
                } else {
                }
            }
        } else {
            let result = recieve_message(&mut stream, buffer, None);
            if let Ok(message) = result {
            } else {
                println!("{}", result.unwrap_err().to_string());
            }
        }
        is_sending = !is_sending;
        thread::sleep(Duration::from_millis(10));
    }
}

fn send_message(stream: &mut TcpStream, message: Message) -> Result<()> {
    let message_string = message.to_string();
    let s = stream.write(&[message as u8])?;
    println!("{}", s);
    stream.flush()?;
    println!("Sent {}...", message_string);
    Ok(())
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
    println!("{:?}", stream.bytes());
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
