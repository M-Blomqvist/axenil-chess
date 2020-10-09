use std::{env, thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Running {}...", args[0]);
    if let Ok((message_sender, handle)) = multiplayer::start_multiplayer(&args[1], &args[2]) {
        for i in 0..6 {
            message_sender
                .send([i; 5])
                .expect("failed in sending message");
            thread::sleep(Duration::from_millis(100))
        }
        handle.join().unwrap();
    }
}
