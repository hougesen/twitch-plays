use crate::keyboard::CommandQueue;
use enigo::Key as EnigoKey;
use std::sync::{Arc, Mutex};

pub struct ParsedMessage {
    pub message_type: CommandType,
    pub response: Option<String>,
    pub key: Option<EnigoKey>,
}

impl ParsedMessage {
    fn new(message_type: CommandType, response: Option<String>, key: Option<EnigoKey>) -> Self {
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
        "!select" => ParsedMessage::new(CommandType::GameCommand, None, Some(EnigoKey::Backspace)),

        "!start" => ParsedMessage::new(CommandType::GameCommand, None, Some(EnigoKey::Return)),

        "!up" => ParsedMessage::new(CommandType::GameCommand, None, Some(EnigoKey::UpArrow)),

        "!down" => ParsedMessage::new(CommandType::GameCommand, None, Some(EnigoKey::DownArrow)),

        "!right" => ParsedMessage::new(CommandType::GameCommand, None, Some(EnigoKey::RightArrow)),

        "!left" => ParsedMessage::new(CommandType::GameCommand, None, Some(EnigoKey::LeftArrow)),

        // Left bumper
        "!l" => ParsedMessage::new(CommandType::GameCommand, None, Some(EnigoKey::Layout('a'))),

        // right bumper
        "!r" => ParsedMessage::new(CommandType::GameCommand, None, Some(EnigoKey::Layout('s'))),

        "!a" => ParsedMessage::new(CommandType::GameCommand, None, Some(EnigoKey::Layout('x'))),

        "!b" => ParsedMessage::new(CommandType::GameCommand, None, Some(EnigoKey::Layout('z'))),

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

pub fn queue_game_command(command_queue: &Arc<Mutex<CommandQueue>>, key: EnigoKey) {
    command_queue.lock().unwrap().enqueue(key);
}
