use std::{env, thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Running {}...", args[0]);
    if let Ok(message_sender) = multiplayer::start_multiplayer(&args[1], &args[2]) {
        loop {
            message_sender
                .send([0x00; 5])
                .expect("failed to send new message");
            thread::sleep(Duration::from_millis(10));
        }
    }
}
