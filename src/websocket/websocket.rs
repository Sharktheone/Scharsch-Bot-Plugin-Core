use jni::JNIEnv;
use jni::objects::{JClass, JValue};
use ws::{connect, Handler, Sender, Result, Message as WSMessage, Handshake, CloseCode};
use crate::config::config_format::Config;
use crate::plugin::logger::{error, info};
use crate::events::message::{Message};


pub struct WSClient<'a> {
    password: String,
    user: String,
    sender: Sender,
    env:&'a JNIEnv<'a>,
    class:&'a JClass<'a>,
}

// static mut WS_CLIENT: Option<WSClient> = None; TODO: Use static value for ws client

impl <'a> Handler for WSClient<'a> {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let client:*const WSClient = self;
        let client_pointer = client as i64;
        let mut env = unsafe { self.env.unsafe_clone() };

        info(&mut env, self.class, "Storing ws pointer".to_string());
        if let Err(err) = store_ws(&mut env, self.class, client_pointer) {
            error(&mut env, self.class, format!("Error storing ws pointer: {}", err));
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
            Err(err) => error(&mut env, self.class, format!("Error sending auth message: {}", err)),
        };
        Ok(())
    }

    fn on_message(&mut self, msg: WSMessage) -> Result<()> {
        println!("Got message: {}", msg);
        self.sender.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Connection closed due to ({:?}) {}", code, reason);
    }
}

fn store_ws(env: &mut JNIEnv, class: &JClass, ptr:i64) ->std::result::Result<(),jni::errors::Error>{
    env.set_field(class, "ws_ptr", "J", JValue::Long(ptr))
}

fn get_ws<'a>(env: &mut JNIEnv<'a>, class: &JClass) ->std::result::Result<*const WSClient<'a>, String>{
    match env.get_field(class, "ws_ptr", "J") {
        Ok(ptr_val) => {
            match ptr_val.j(){
                Ok(ptr_j) => {
                    if ptr_j == 0 {
                        Err("No ws pointer stored".to_string())
                    } else {
                        let ws_ptr:*const WSClient = ptr_j as *const WSClient;
                        if ws_ptr.is_null() {
                            Err("Null ws pointer".to_string())
                        } else {
                            Ok(ws_ptr)
                        }
                    }
                }
                Err(err) => Err(format!("Error getting ws pointer: {}", err))
            }
        },
        Err(err) => Err(format!("Error getting ws pointer: {}", err)),
    }
}

pub fn connect_ws(env: &mut JNIEnv, class: &JClass, config: Config) ->std::result::Result<(), String> {
    let url = format!("{}://{}:{}/{}/ws", config.protocol, config.host, config.port, config.serverid);

    match connect(url, |sender| {
        WSClient {
            password: config.password.to_string(),
            user: config.user.to_string(),
            sender,
            env: &env,
            class:&class,
        }
    }) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error connecting to ws: {}", err)),
    }
}

pub fn send(env: &mut JNIEnv, class: &JClass, msg: Message) -> std::result::Result<(), String> {
    match get_ws(env, class) {
        Ok(ws_ptr) => {
            let ws: &WSClient = unsafe { &*ws_ptr };
            return match serde_json::to_string(&msg) {
                Ok(text) => {
                    match ws.sender.send(text) {
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