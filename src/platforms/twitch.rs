use crate::keyboard::KeyboardController;
use crate::message_parser::{parse_chat_message, queue_game_command, CommandType};
use std::sync::{Arc, Mutex};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;
use tungstenite::{connect, Message};
use url::Url;

pub struct TwitchCredentials {
    channel: String,
    access_token: String,
}

impl TwitchCredentials {
    #[allow(unused)]
    fn new(channel: String, access_token: String) -> Self {
        TwitchCredentials {
            channel,
            access_token,
        }
    }

    fn load_from_env() -> Self {
        use dotenv::dotenv;

        dotenv().ok();

        let channel = dotenv::var("TWITCH_CHANNEL_NAME").expect("Missing: TWITCH_CHANNEL_NAME env");

        let access_token =
            dotenv::var("TWITCH_ACCESS_TOKEN").expect("Missing: TWITCH_ACCESS_TOKEN env");

        TwitchCredentials {
            channel,
            access_token,
        }
    }
}

pub struct TwitchChat {
    credentials: TwitchCredentials,
    socket: WebSocket<MaybeTlsStream<std::net::TcpStream>>,
}

impl TwitchChat {
    pub fn new(credentials: Option<TwitchCredentials>) -> Self {
        if let Some(credentials) = credentials {
            return TwitchChat {
                socket: TwitchChat::start_socket(&credentials).expect("Error starting websocket"),
                credentials,
            };
        }

        let credentials = TwitchCredentials::load_from_env();

        TwitchChat {
            socket: TwitchChat::start_socket(&credentials).expect("Error starting websocket"),
            credentials,
        }
    }

    pub fn start_socket(
        credentials: &TwitchCredentials,
    ) -> Result<WebSocket<MaybeTlsStream<std::net::TcpStream>>, tungstenite::Error> {
        println!("Connecting to Twitch");

        let (mut socket, _response) =
            connect(Url::parse("wss://irc-ws.chat.twitch.tv:443").unwrap()).expect("Can't connect");

        socket.write_message(Message::Text(format!(
            "PASS oauth:{}",
            &credentials.access_token
        )))?;

        socket.write_message(Message::Text(format!("NICK {}", &credentials.channel)))?;

        println!("send token & loginname");

        socket.write_message(Message::Text(format!("JOIN #{}", &credentials.channel)))?;

        Ok(socket)
    }

    pub fn read_chat(&mut self, keyboard_controller: Arc<Mutex<KeyboardController>>) {
        loop {
            let msg = self
                .socket
                .read_message()
                .expect("Error reading message")
                .to_text()
                .unwrap()
                .to_owned();

            println!("Received: {}", msg);

            if msg.contains("PING") {
                self.socket
                    .write_message(Message::Text(String::from("PONG")))
                    .expect("Error sending PONG to Twitch");
            } else if msg.contains("PRIVMSG") {
                // :caveaio!caveaio@caveaio.tmi.twitch.tv PRIVMSG #hougesen :test

                let (_sender, message) = msg.split_once('!').unwrap();

                let (_, chat_message) = message.split_once(':').unwrap();

                println!("Message: {}", &chat_message);

                let parsed_message = parse_chat_message(chat_message);

                match parsed_message.message_type {
                    CommandType::GameCommand => {
                        if let Some(key) = parsed_message.key {
                            queue_game_command(&keyboard_controller, key);
                        }
                    }
                    CommandType::ChannelCommand => {
                        if let Some(response) = parsed_message.response {
                            self.queue_text_message(response)
                                .expect("Error sending chat message");
                        }
                    }
                    CommandType::Unknown => (),
                };
            }
        }
    }

    fn queue_text_message(&mut self, message: String) -> Result<(), tungstenite::Error> {
        self.socket.write_message(Message::Text(format!(
            "PRIVMSG #{} :{message}",
            &self.credentials.channel
        )))?;

        Ok(())
    }
}
