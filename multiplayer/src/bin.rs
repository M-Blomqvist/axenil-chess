use std::{env, thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Running {}...", args[0]);
    multiplayer::start_multiplayer(&args[1], &args[2]);
}
