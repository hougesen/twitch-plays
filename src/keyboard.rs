use enigo::{Enigo, KeyboardControllable};
use std::sync::{Arc, Mutex};

pub struct CommandQueue(Vec<enigo::Key>);

impl CommandQueue {
    pub fn new() -> Self {
        CommandQueue(vec![])
    }

    pub fn enqueue(&mut self, key: enigo::Key) {
        self.0.push(key);
    }

    pub fn dequeue(&mut self) -> Option<enigo::Key> {
        if !self.0.is_empty() {
            return Some(self.0.remove(0));
        }

        None
    }
}

pub struct KeyboardController {
    pub command_queue: Arc<Mutex<CommandQueue>>,
    enigo: Enigo,
}

impl KeyboardController {
    pub fn new() -> Self {
        KeyboardController {
            command_queue: Arc::new(Mutex::new(CommandQueue::new())),
            enigo: Enigo::new(),
        }
    }

    pub fn next_command(&mut self) {
        let command = self.command_queue.lock().unwrap().dequeue();

        if let Some(cmd) = command {
            self.press_key(cmd);
        }
    }

    pub fn press_key(&mut self, key: enigo::Key) {
        self.enigo.key_down(key);
        // Have to sleep for atleast 200ms between key presses, otherwise the emulator will see it as just turning
        std::thread::sleep(std::time::Duration::from_millis(200));
        self.enigo.key_up(key);
    }

    /// Saves the game using mGBA's keyboard shortcut (SHIFT+F1)
    pub fn save_game_state(&mut self) {
        println!("Saving game state");

        self.enigo.key_down(enigo::Key::Shift);
        self.enigo.key_click(enigo::Key::F1);
        self.enigo.key_up(enigo::Key::Shift);
    }
}
