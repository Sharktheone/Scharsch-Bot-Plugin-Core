use crate::events::message::Message;
use crate::plugin::logger::{error};
use crate::events::message::{SEND_PLAYERS, WHITELIST_ADD, WHITELIST_REMOVE, AUTH_SUCCESS, WHITELISTED_PLAYERS};
use crate::events::ws_events::{auth_success, send_players, whitelist_add, whitelist_remove, whitelisted_players};

pub(crate) static mut HANDLERS: Option<Handlers> = None;

pub(crate) struct Handlers {
    pub(crate) get_players_handler: Option<&'static dyn Fn() -> Result<Vec<String>, String>>,
    pub(crate) add_whitelist: Option<&'static dyn Fn(String) -> Result<(), String>>,
    pub(crate) remove_whitelist: Option<&'static dyn Fn(String) -> Result<(), String>>,
    pub(crate) whitelisted_players: Option<&'static dyn Fn() -> Result<Vec<String>, String>>
}

pub fn set_handlers(get_players_handler: Option<&'static dyn Fn() -> Result<Vec<String>, String>>, add_whitelist: Option<&'static dyn Fn(String) -> Result<(), String>>, remove_whitelist: Option<&'static dyn Fn(String) -> Result<(), String>>, whitelisted_players: Option<&'static dyn Fn() -> Result<Vec<String>, String>>) {
    let handlers: Handlers = Handlers {
        get_players_handler,
        add_whitelist,
        remove_whitelist,
        whitelisted_players
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
        WHITELIST_ADD => whitelist_add(message),
        WHITELIST_REMOVE => whitelist_remove(message),
        AUTH_SUCCESS => auth_success(),
        WHITELISTED_PLAYERS => whitelisted_players(),

        _ => {
            error(format!("Unknown event: {}", message.event));
        }

    }
}