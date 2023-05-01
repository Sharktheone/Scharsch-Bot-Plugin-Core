use crate::events::message::{Message, SEND_PLAYERS, KICK_PLAYER, REPORT_PLAYER, BAN_PLAYER, UNBAN_PLAYER, SEND_COMMAND, SEND_CHAT_MESSAGE, WHITELIST_ADD, WHITELIST_REMOVE, WHITELISTED_PLAYERS, AUTH_SUCCESS, AUTH_FAILED};
use crate::plugin::logger::{error};
use crate::events::ws_events::{send_players, kick_player, report_player, ban_player, unban_player, send_command, send_chat_message, whitelist_add, whitelist_remove, whitelisted_players, auth_success, auth_failed};

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
        KICK_PLAYER => kick_player(message),
        REPORT_PLAYER => report_player(message),
        BAN_PLAYER => ban_player(message),
        UNBAN_PLAYER => unban_player(message),
        SEND_COMMAND => send_command(message),
        SEND_CHAT_MESSAGE => send_chat_message(message),
        WHITELIST_ADD => whitelist_add(message),
        WHITELIST_REMOVE => whitelist_remove(message),
        WHITELISTED_PLAYERS => whitelisted_players(),
        AUTH_SUCCESS => auth_success(),
        AUTH_FAILED => auth_failed(),


        _ => {
            error(format!("Unknown event: {}", message.event));
        }

    }
}