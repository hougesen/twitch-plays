use crate::{pokemon::Pokemon, twitch::get_twitch_credentials};
use enigo::Key as EnigoKey;
use std::sync::{Arc, Mutex};
use tungstenite::{connect, Message};
use url::Url;

pub struct ParsedMessage {
    message_type: CommandType,
    response: Option<String>,
    key: Option<EnigoKey>,
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

pub fn start_socket(game: Arc<Mutex<Pokemon>>) -> Result<(), tungstenite::Error> {
    println!("Connecting to Twitch");

    let (mut socket, _response) =
        connect(Url::parse("wss://irc-ws.chat.twitch.tv:443").unwrap()).expect("Can't connect");

    let (channel, token) = get_twitch_credentials().unwrap();

    socket.write_message(Message::Text(format!("PASS oauth:{}", &token)))?;
    socket.write_message(Message::Text(format!("NICK {}", &channel)))?;

    println!("send token & loginname");

    socket.write_message(Message::Text(format!("JOIN #{}", &channel)))?;

    loop {
        let msg = socket
            .read_message()
            .expect("Error reading message")
            .to_text()
            .unwrap()
            .to_owned();

        println!("Received: {}", msg);

        if msg.contains("PING") {
            socket.write_message(Message::Text(String::from("PONG")))?;
        } else if msg.contains("PRIVMSG") {
            // :caveaio!caveaio@caveaio.tmi.twitch.tv PRIVMSG #hougesen :test

            let (_sender, message) = msg.split_once('!').unwrap();

            let (_, chat_message) = message.split_once(':').unwrap();

            println!("Message: {}", &chat_message);
            let parsed_message = parse_chat_message(chat_message);

            match parsed_message.message_type {
                CommandType::GameCommand => {
                    if let Some(key) = parsed_message.key {
                        queue_game_command(&game, key);
                    }
                }
                CommandType::ChannelCommand => {
                    if let Some(response) = parsed_message.response {
                        queue_text_message(&mut socket, &channel, response)?;
                    }
                }
                CommandType::Unknown => (),
            };
        }
    }
}

pub fn parse_chat_message<S: ToString>(message: S) -> ParsedMessage {
    return match message.to_string().trim().to_lowercase().as_str() {
        // Pokemon game commands
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

fn queue_game_command(game: &Arc<Mutex<Pokemon>>, key: EnigoKey) {
    game.lock().unwrap().queue_command(key);
}

fn queue_text_message(
    socket: &mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>,
    channel: &String,
    message: String,
) -> Result<(), tungstenite::Error> {
    socket.write_message(Message::Text(format!("PRIVMSG #{channel} :{message}")))?;

    Ok(())
}
