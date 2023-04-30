use ws::{connect, Handler, Sender, Result, Message as WSMessage, Handshake, CloseCode};
use crate::config::config_format::Config;
use crate::events::handler::handle_message;
use crate::plugin::logger::{error_no_env, info};
use crate::events::message::{Message};


pub struct WSClient {
    password: String,
    user: String,
    sender: Sender,
}

static mut WS_CLIENT: Option<WSClient> = None;

impl Handler for WSClient {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        info("Storing ws pointer".to_string());

        unsafe {
            let client = WSClient {
                password: self.password.clone(),
                user: self.user.clone(),
                sender: self.sender.clone(),
            };
            WS_CLIENT = Some(client);
        }

        let auth_base= r#"
        {
            "event": "auth",
            "data": {
                "user": "{user}",
                "password": "{password}"
                }
        }
        "#;

        let mut auth = auth_base.to_string();

        auth = auth.replace("{user}", &mut self.user);
        auth = auth.replace("{password}", &mut self.password);


        match self.sender.send(auth){
            Ok(_) => {},
            Err(err) => error_no_env(format!("Error sending auth message: {}", err)),
        };
        Ok(())
    }

    fn on_message(&mut self, msg: WSMessage) -> Result<()> {

        handle_message(msg.to_string());
        println!("Got message: {}", msg);
        self.sender.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Connection closed due to ({:?}) {}", code, reason);
    }
}


fn get_ws<'a>() ->std::result::Result<&'static mut WSClient, String>{
    unsafe {
        match WS_CLIENT.as_mut() {
            Some(ws) => Ok(ws),
            None => Err("No ws client".to_string()),
        }
    }
}

pub fn connect_ws(config: Config) ->std::result::Result<(), String> {
    let url = format!("{}://{}:{}/{}/ws", config.protocol, config.host, config.port, config.serverid);

    match connect(url, |sender| {
        WSClient {
            password: config.password.to_string(),
            user: config.user.to_string(),
            sender,
        }
    }) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error connecting to ws: {}", err)),
    }
}

pub fn send(msg: Message) -> std::result::Result<(), String> {
    match get_ws() {
        Ok(client) => {
            return match serde_json::to_string(&msg) {
                Ok(text) => {
                    match client.sender.send(text) {
                        Ok(_) => Ok(()),
                        Err(err) => Err(format!("Error sending message: {}", err)),
                    }
                },
                Err(err) => {
                    Err(format!("Error converting message to text: {}", err))
                },
            }
        }
        Err(err) => Err(format!("Error getting ws: {}", err)),
    }
}