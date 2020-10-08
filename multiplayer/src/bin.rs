use std::{env, thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Running {}...", args[0]);
    if let Ok((message_sender, thread_handle)) = multiplayer::start_multiplayer(&args[1], &args[2])
    {
        thread_handle.join().unwrap();
    }
}
