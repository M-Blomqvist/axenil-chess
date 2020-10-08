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

pub fn start_multiplayer(
    connect_type: &str,
    ip: &str,
) -> Result<(Sender<[u8; 5]>, JoinHandle<()>)> {
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

fn start_host(ip: SocketAddrV4) -> Result<(Sender<[u8; 5]>, JoinHandle<()>)> {
    let listener = TcpListener::bind(ip).unwrap();
    println!("Started host at {}", ip);

    if let Ok((mut stream, client)) = listener.accept() {
        println!("Connection from {}", client);
        stream.set_read_timeout(Some(Duration::from_secs(1)))?;
        stream.set_write_timeout(Some(Duration::from_secs(1)))?;

        let buffer = [255; 5];
        recieve_message(&mut stream, buffer, Some(Message::Accept))?;
        send_message(&mut stream, Message::Accept)?;

        let (sender, reciever) = channel::<[u8; 5]>();
        let handle = thread::spawn(move || {
            std_loop(stream, reciever);
        });
        Ok((sender, handle))
    } else {
        Err(Error::new(
            ErrorKind::Other,
            "Error setting up host & sender".to_string(),
        ))
    }
}

fn connect_client(ip: SocketAddrV4) -> Result<(Sender<[u8; 5]>, JoinHandle<()>)> {
    if let Ok(mut stream) = TcpStream::connect(ip) {
        println!("Connection established to: {}", ip);

        send_message(&mut stream, Message::Accept)?;

        let buffer = [255; 5];
        recieve_message(&mut stream, buffer, Some(Message::Accept))?;

        let (sender, reciever) = channel::<[u8; 5]>();
        let handle = thread::spawn(move || {
            std_loop(stream, reciever);
        });
        Ok((sender, handle))
    } else {
        panic!("Error connecting to {}", ip);
    }
}

fn std_loop(mut stream: TcpStream, rx: Receiver<[u8; 5]>) {
    loop {
        let buffer = [255; 5];
        println!("h");
        let mut counter = 0;
        if let Ok(message) = rx.try_recv() {
            counter = 0;
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
                if message != [255; 5] {
                    counter = 0;
                } else {
                    counter += 1;
                    println!("counter");
                    if counter > 1000 {
                        break;
                    }
                }
            } else {
                println!("{}", result.unwrap_err().to_string());
            }
        }
        thread::sleep(Duration::from_millis(1));
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
    if let Ok(len) = stream.read(&mut buffer[..]) {
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
        let mut buff = [0 as u8; 1024];
        let n = stream.read(&mut buff[..]).unwrap();
        println!("{}: {:?}", n, stream.bytes());
        panic!("Error reading stream!");
    }
}
