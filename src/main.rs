mod keyboard;
mod message_parser;
mod platforms;

use keyboard::KeyboardController;
use platforms::twitch::TwitchChat;
use std::sync::{Arc, Mutex};

fn main() {
    let keyboard_controller = Arc::new(Mutex::new(KeyboardController::new()));

    let twitch_chat_clone = Arc::clone(&keyboard_controller);

    std::thread::spawn(move || {
        let mut twitch_chat = TwitchChat::new(None);

        twitch_chat.read_chat(twitch_chat_clone);
    });

    let queue_clone = Arc::clone(&keyboard_controller);

    loop {
        queue_clone.lock().unwrap().next_command();

        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
