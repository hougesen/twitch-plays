use enigo::{Enigo, Key as EnigoKey, KeyboardControllable};

pub struct CommandQueue(Vec<EnigoKey>);

impl CommandQueue {
    pub fn new() -> Self {
        CommandQueue(vec![])
    }

    pub fn enqueue(&mut self, key: EnigoKey) {
        self.0.push(key);
    }

    pub fn dequeue(&mut self) -> Option<EnigoKey> {
        if !self.0.is_empty() {
            return Some(self.0.remove(0));
        }

        None
    }
}

pub struct Pokemon {
    pub enigo: Enigo,
    pub command_queue: CommandQueue,
}

impl Pokemon {
    pub fn new() -> Self {
        Pokemon {
            enigo: Enigo::new(),
            command_queue: CommandQueue::new(),
        }
    }

    pub fn queue_command(&mut self, command: EnigoKey) {
        self.command_queue.enqueue(command)
    }

    pub fn next_command(&mut self) {
        let command = self.command_queue.dequeue();

        if let Some(cmd) = command {
            self.press_key(cmd)
        }
    }

    pub fn press_key(&mut self, key: EnigoKey) {
        self.enigo.key_down(key);
        // Have to sleep for atleast 200ms between key presses, otherwise the emulator will see it as just turning
        std::thread::sleep(std::time::Duration::from_millis(200));
        self.enigo.key_up(key);
    }
}
