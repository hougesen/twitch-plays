use crate::keyboard::CommandQueue;
use std::sync::{Arc, Mutex};

pub struct ParsedMessage {
    pub message_type: CommandType,
    pub response: Option<String>,
    pub key: Option<enigo::Key>,
}

impl ParsedMessage {
    fn new(message_type: CommandType, response: Option<String>, key: Option<enigo::Key>) -> Self {
        ParsedMessage {
            message_type,
            response,
            key,
        }
    }
}

pub enum CommandType {
    GameCommand,
    ChannelCommand,
    Unknown,
}

pub fn parse_chat_message<S: ToString>(message: S) -> ParsedMessage {
    return match message.to_string().trim().to_lowercase().as_str() {
        // Game commands
        "!select" => {
            ParsedMessage::new(CommandType::GameCommand, None, Some(enigo::Key::Backspace))
        }

        "!start" => ParsedMessage::new(CommandType::GameCommand, None, Some(enigo::Key::Return)),

        "!up" => ParsedMessage::new(CommandType::GameCommand, None, Some(enigo::Key::UpArrow)),

        "!down" => ParsedMessage::new(CommandType::GameCommand, None, Some(enigo::Key::DownArrow)),

        "!right" => {
            ParsedMessage::new(CommandType::GameCommand, None, Some(enigo::Key::RightArrow))
        }

        "!left" => ParsedMessage::new(CommandType::GameCommand, None, Some(enigo::Key::LeftArrow)),

        // Left bumper
        "!l" => ParsedMessage::new(
            CommandType::GameCommand,
            None,
            Some(enigo::Key::Layout('a')),
        ),

        // right bumper
        "!r" => ParsedMessage::new(
            CommandType::GameCommand,
            None,
            Some(enigo::Key::Layout('s')),
        ),

        "!a" => ParsedMessage::new(
            CommandType::GameCommand,
            None,
            Some(enigo::Key::Layout('x')),
        ),

        "!b" => ParsedMessage::new(
            CommandType::GameCommand,
            None,
            Some(enigo::Key::Layout('z')),
        ),

        // Chat commands
        "!github" | "!code" => ParsedMessage::new(
            CommandType::ChannelCommand,
            Some("You can find the code @ https://github.com/hougesen/twitch-plays".to_string()),
            None,
        ),

        "!help" | "!commands" => ParsedMessage::new(
            CommandType::ChannelCommand,
            Some(
                "You can find the commands @ https://github.com/hougesen/twitch-plays#commands"
                    .to_string(),
            ),
            None,
        ),

        // Unknown command
        _ => ParsedMessage::new(CommandType::Unknown, None, None),
    };
}

pub fn queue_game_command(command_queue: &Arc<Mutex<CommandQueue>>, key: enigo::Key) {
    command_queue.lock().unwrap().enqueue(key);
}
