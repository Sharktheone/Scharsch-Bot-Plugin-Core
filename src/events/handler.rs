use crate::events::message::Message;
use crate::plugin::logger::{error};
use crate::events::message::{SEND_PLAYERS};
use crate::events::ws_events::send_players;

pub(crate) static mut HANDLERS: Option<Handlers> = None;

pub(crate) struct Handlers {
    pub(crate) get_players_handler: Option<&'static dyn Fn() -> Result<Vec<String>, String>>,
    pub(crate) add_whitelist: Option<&'static dyn Fn(String) -> Result<(), String>>,
    pub(crate) remove_whitelist: Option<&'static dyn Fn(String) -> Result<(), String>>
}

pub fn set_handlers(get_players_handler: Option<&'static dyn Fn() -> Result<Vec<String>, String>>, add_whitelist: Option<&'static dyn Fn(String) -> Result<(), String>>, remove_whitelist: Option<&'static dyn Fn(String) -> Result<(), String>>) {
    let handlers: Handlers = Handlers {
        get_players_handler,
        add_whitelist,
        remove_whitelist
    };

    unsafe {
        HANDLERS = Some(handlers);
    }
}

pub(crate) fn handle_message(msg: String) {
    let message: Message = match serde_json::from_str(&msg) {
        Ok(message) => message,
        Err(err) => {
            error(format!("Error parsing message: {}", err));
            return;
        }
    };

    match message.event {
        SEND_PLAYERS => send_players(),
        _ => {
            error(format!("Unknown event: {}", message.event));
        }

    }
}