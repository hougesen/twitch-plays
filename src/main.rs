mod message_parser;
mod platforms;
mod pokemon;

use platforms::twitch::TwitchChat;
use pokemon::Pokemon;
use std::sync::{Arc, Mutex};

fn main() {
    let game = Arc::new(Mutex::new(Pokemon::new()));

    let twitch_chat_clone = Arc::clone(&game);

    std::thread::spawn(move || {
        let mut twitch_chat = TwitchChat::new(None);

        twitch_chat.read_chat(twitch_chat_clone);
    });

    let queue_clone = Arc::clone(&game);

    loop {
        queue_clone.lock().unwrap().next_command();

        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
