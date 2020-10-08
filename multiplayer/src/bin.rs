use std::{env, thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Running {}...", args[0]);
    if let Ok(message_sender) = multiplayer::start_multiplayer(&args[1], &args[2]) {
        for i in 0..5 {
            message_sender
                .send([i; 5])
                .expect("failed to send new message");
            thread::sleep(Duration::from_millis(10));
        }
    }
}
