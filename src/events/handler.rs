use jni::JNIEnv;
use jni::objects::{JClass};
use crate::events::message::Message;
use crate::plugin::logger::{error};
use crate::events::message::{SEND_PLAYERS};
use crate::events::ws_events::send_players;

pub(crate) static mut HANDLERS: Option<Handlers> = None;

pub(crate) struct Handlers {
    pub(crate) get_players_handler: Option<&'static dyn Fn() -> Result<Vec<String>, String>>,
}

pub fn set_handlers(get_players_handler: Option<&'static dyn Fn() -> Result<Vec<String>, String>>) {
    let handlers: Handlers = Handlers {
        get_players_handler,
    };

    unsafe {
        HANDLERS = Some(handlers);
    }

}

pub(crate) fn handle_message(msg: String, env: &mut JNIEnv, class: &JClass) {
    let message: Message = match serde_json::from_str(&msg) {
        Ok(message) => message,
        Err(err) => {
            error(env, class, format!("Error parsing message: {}", err));
            return;
        }
    };

    match message.event {
        SEND_PLAYERS => send_players(env, class),
        _ => {
            error(env, class, format!("Unknown event: {}", message.event));
        }

    }
}