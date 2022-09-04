use crate::{
    message_parser::{parse_chat_message, queue_game_command, CommandType},
    pokemon::Pokemon,
};
use dotenv::dotenv;
use std::sync::{Arc, Mutex};
use tungstenite::{connect, Message};
use url::Url;

pub fn get_twitch_credentials() -> Result<(String, String), dotenv::Error> {
    dotenv().ok();

    let channel_name = dotenv::var("TWITCH_CHANNEL_NAME")?;

    let access_token = dotenv::var("TWITCH_ACCESS_TOKEN")?;

    Ok((channel_name, access_token))
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

fn queue_text_message(
    socket: &mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>,
    channel: &String,
    message: String,
) -> Result<(), tungstenite::Error> {
    socket.write_message(Message::Text(format!("PRIVMSG #{channel} :{message}")))?;

    Ok(())
}
