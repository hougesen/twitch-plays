mod message_parser;
mod platforms;
mod pokemon;

use platforms::twitch::start_socket;
use pokemon::Pokemon;
use std::sync::{Arc, Mutex};

fn main() {
    let game = Arc::new(Mutex::new(Pokemon::new()));

    let socket_clone = Arc::clone(&game);

    std::thread::spawn(move || start_socket(socket_clone));

    let queue_clone = Arc::clone(&game);

    loop {
        queue_clone.lock().unwrap().next_command();

        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
