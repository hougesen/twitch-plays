mod keyboard;
mod message_parser;
mod platforms;

use keyboard::KeyboardController;
use platforms::twitch::TwitchChat;
use std::sync::Arc;

fn main() {
    let mut keyboard_controller = KeyboardController::new();

    let twitch_chat_clone = Arc::clone(&keyboard_controller.command_queue);

    std::thread::spawn(move || {
        let mut twitch_chat = TwitchChat::new(None);

        twitch_chat.read_chat(twitch_chat_clone);
    });

    let mut last_save = std::time::Instant::now();

    loop {
        keyboard_controller.next_command();

        std::thread::sleep(std::time::Duration::from_millis(20));

        // Save game every 5 minutes
        if last_save.elapsed().as_secs() == 300 {
            keyboard_controller.save_game_state();
            last_save = std::time::Instant::now();
        }
    }
}
