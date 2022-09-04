mod pokemon;

use pokemon::Pokemon;
use std::sync::{Arc, Mutex};

fn main() {
    let game = Arc::new(Mutex::new(Pokemon::new()));

    let queue_clone = Arc::clone(&game);

    loop {
        queue_clone.lock().unwrap().next_command();

        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
