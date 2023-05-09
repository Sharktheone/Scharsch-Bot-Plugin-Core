// Bot => Server: Error

use crate::events::handler::{HANDLERS, Handlers};
use crate::events::message::{ERROR, Message, MessageData, PLAYERS};
use crate::plugin::logger::{warn};
use crate::websocket::websocket::{send, AUTHENTICATED};

fn get_handlers() -> Result<&'static Handlers, ()> {
    unsafe {
        match HANDLERS.as_ref() {
            Some(handlers) => Ok(handlers),
            None => {
                let msg = Message {
                    event: ERROR,
                    data: MessageData {
                        error: Some("No handlers implemented".to_string()),
                        ..MessageData::default()
                    },
                };
                match send( msg) {
                    Ok(_) => {},
                    Err(err) => {warn(format!(r#"Error sending: "No handlers implemented" : {}"#, err)) },
                };
                Err(())
            }
        }
    }
}

pub(crate) fn send_players() {
    if let Ok(handlers) = get_handlers() { match handlers.get_players_handler {
        Some(get_players_handler) => match (get_players_handler)() {
            Ok(players) => {
                let msg = Message {
                    event: PLAYERS,
                    data: MessageData {
                        players: Some(players),
                        ..MessageData::default()
                    },
                };

                match send(msg) {
                    Ok(_) => {}
                    Err(err) => warn(format!("Error sending players: {}", err)),
                };
            }
            Err(err) => {
                let msg = Message {
                    event: ERROR,
                    data: MessageData {
                        error: Some(format!("Error getting players: {}", err)),
                        ..MessageData::default()
                    },
                };
                match send(msg) {
                    Ok(_) => {}
                    Err(err) => warn(format!(r#"Error sending: "Error getting players": {}"#, err)),
                };
            }
        },
        None => {
            let msg = Message {
                event: ERROR,
                data: MessageData {
                    error: Some("No get players handler implemented".to_string()),
                    ..MessageData::default()
                },
            };
            match send(msg) {
                Ok(_) => {}
                Err(err) => warn(format!(r#"Error sending: "No get players handler implemented" : {}"#, err)),
            };
        }
    } }
}

pub(crate) fn get_player(message: Message) -> Result<String, ()> {
    match message.data.player {
        Some(player) => Ok(player),
        None => {
            match send(Message {
                event: ERROR,
                data: MessageData {
                    error: Some("No player name provided".to_string()),
                    ..MessageData::default()
                },
            }) {
                Ok(_) => {},
                Err(err) => { warn(format!(r#"Error sending: "No player name provided" : {}"#, err)) },
            }
            Err(())
        }
    }
}

pub(crate) fn get_uuid(message: Message) -> Result<String, ()> {
    match message.data.uuid {
        Some(uuid) => Ok(uuid),
        None => {
            match send(Message {
                event: ERROR,
                data: MessageData {
                    error: Some("No player uuid provided".to_string()),
                    ..MessageData::default()
                },
            }) {
                Ok(_) => {},
                Err(err) => { warn(format!(r#"Error sending: "No player uuid provided" : {}"#, err)) },
            }
            Err(())
        }
    }
}


pub(crate) fn kick_player(message: Message) {
    if let Ok(handlers) = get_handlers() { match handlers.kick_player {
        Some(kick_player) => {
            let player = match get_player(message.clone()) {
                Ok(player) => player,
                Err(_) => return,
            };
            let reason = message.data.reason.unwrap_or("No reason provided".to_string());
            let is_component = message.data.message_is_component.unwrap_or(false);
            match (kick_player)(player, reason, is_component) {
                Ok(_) => {}
                Err(err) => warn(format!("Error kicking player: {}", err)),
            }
        },
        None => {
            let msg = Message {
                event: ERROR,
                data: MessageData {
                    error: Some("No kick player handler implemented".to_string()),
                    ..MessageData::default()
                },
            };
            match send(msg) {
                Ok(_) => {}
                Err(err) => warn(format!(r#"Error sending: "No kick player handler implemented" : {}"#, err)),
            };
        }
    } }

}

pub(crate) fn report_player(message: Message) {
    if let Ok(handlers) = get_handlers() { match handlers.send_admin_message {
        Some(send_admin_message) => {
            let message = match message.data.message {
                Some(message) => message,
                None => {
                    match send(Message {
                        event: ERROR,
                        data: MessageData {
                            error: Some("Cannot send report: No message provided".to_string()),
                            ..MessageData::default()
                        },
                    }) {
                        Ok(_) => {},
                        Err(err) => { warn(format!(r#"Error sending: "Cannot send report: No message provided" : {}"#, err)) },
                    }
                    return;
                }
            };
            match (send_admin_message)(message) {
                Ok(_) => {}
                Err(err) => warn(format!("Error reporting player: {}", err)),
            }
        },
        None => {
            let msg = Message {
                event: ERROR,
                data: MessageData {
                    error: Some("No report player handler implemented".to_string()),
                    ..MessageData::default()
                },
            };
            match send(msg) {
                Ok(_) => {}
                Err(err) => warn(format!(r#"Error sending: "No report player handler implemented" : {}"#, err)),
            };
        }
    } }
}

pub(crate) fn ban_player(message: Message) {
    if let Ok(handlers) = get_handlers() { match handlers.ban_player {
        Some(ban_player) => {
            let player = match get_player(message.clone()) {
                Ok(player) => player,
                Err(_) => return,
            };
            let reason = message.data.reason.unwrap_or("No reason provided".to_string());
            let is_component = message.data.message_is_component.unwrap_or(false);
            match (ban_player)(player, reason, is_component) {
                Ok(_) => {}
                Err(err) => warn(format!("Error banning player: {}", err)),
            }
        },
        None => {
            let msg = Message {
                event: ERROR,
                data: MessageData {
                    error: Some("No ban player handler implemented".to_string()),
                    ..MessageData::default()
                },
            };
            match send(msg) {
                Ok(_) => {}
                Err(err) => warn(format!(r#"Error sending: "No ban player handler implemented" : {}"#, err)),
            };
        }
    } }

}

pub(crate) fn unban_player(message: Message) {
    if let Ok(handlers) = get_handlers() { match handlers.unban_player {
        Some(unban_player) => {
            let player = match get_player(message.clone()) {
                Ok(player) => player,
                Err(_) => return,
            };
            match (unban_player)(player) {
                Ok(_) => {}
                Err(err) => warn(format!("Error unbanning player: {}", err)),
            }
        },
        None => {
            let msg = Message {
                event: ERROR,
                data: MessageData {
                    error: Some("No unban player handler implemented".to_string()),
                    ..MessageData::default()
                },
            };
            match send(msg) {
                Ok(_) => {}
                Err(err) => warn(format!(r#"Error sending: "No unban player handler implemented" : {}"#, err)),
            };
        }
    } }


}

pub(crate) fn send_command(message: Message) {
    if let Ok(handlers) = get_handlers() { match handlers.send_command {
        Some(send_command) => {
            let command = match message.data.command {
                Some(command) => command,
                None => {
                    match send(Message {
                        event: ERROR,
                        data: MessageData {
                            error: Some("No command provided".to_string()),
                            ..MessageData::default()
                        },
                    }) {
                        Ok(_) => {},
                        Err(err) => { warn(format!(r#"Error sending: "No command provided" : {}"#, err)) },
                    }
                    return;
                }
            };
            match (send_command)(command) {
                Ok(_) => {}
                Err(err) => warn(format!("Error sending command: {}", err)),
            }
        }
            None => {
                let msg = Message {
                    event: ERROR,
                    data: MessageData {
                        error: Some("No send command handler implemented".to_string()),
                        ..MessageData::default()
                    },
                };
                match send(msg) {
                    Ok(_) => {}
                    Err(err) => warn(format!(r#"Error sending: "No send command handler implemented" : {}"#, err)),
                };
            }
    } }


}

pub(crate) fn send_chat_message(message: Message) {
    if let Ok(handlers) = get_handlers() { match handlers.send_message {
        Some(send_chat_message) => {
            let message = match message.data.message {
                Some(message) => message,
                None => {
                    match send(Message {
                        event: ERROR,
                        data: MessageData {
                            error: Some("No chat message provided".to_string()),
                            ..MessageData::default()
                        },
                    }) {
                        Ok(_) => {},
                        Err(err) => { warn(format!(r#"Error sending chat message error: "No message provided" : {}"#, err)) },
                    }
                    return;
                }
            };
            match (send_chat_message)(message) {
                Ok(_) => {}
                Err(err) => warn(format!("Error sending chat message: {}", err)),
            }
        }
        None => {
            let msg = Message {
                event: ERROR,
                data: MessageData {
                    error: Some("No send chat message handler implemented".to_string()),
                    ..MessageData::default()
                },
            };
            match send(msg) {
                Ok(_) => {}
                Err(err) => warn(format!(r#"Error sending: "No send chat message handler implemented" : {}"#, err)),
            };
        }
    } }
}

pub(crate) fn send_admin_message(message: Message) {
    if let Ok(handlers) = get_handlers() { match handlers.send_admin_message {
        Some(send_admin_message) => {
            let message = match message.data.message {
                Some(message) => message,
                None => {
                    match send(Message {
                        event: ERROR,
                        data: MessageData {
                            error: Some("No admin message provided".to_string()),
                            ..MessageData::default()
                        },
                    }) {
                        Ok(_) => {},
                        Err(err) => { warn(format!(r#"Error sending admin message error: "No message provided" : {}"#, err)) },
                    }
                    return;
                }
            };
            match (send_admin_message)(message) {
                Ok(_) => {}
                Err(err) => warn(format!("Error sending admin message: {}", err)),
            }
        }
        None => {
            let msg = Message {
                event: ERROR,
                data: MessageData {
                    error: Some("No send admin message handler implemented".to_string()),
                    ..MessageData::default()
                },
            };
            match send(msg) {
                Ok(_) => {}
                Err(err) => warn(format!(r#"Error sending: "No send admin message handler implemented" : {}"#, err)),
            };
        }
    } }
}

pub(crate) fn whitelist_add(message: Message) {
    let name = match get_player(message.clone()) {
        Ok(name) => name,
        Err(_) => return,
    };
    let uuid = match get_uuid(message.clone()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return;
        }
    };
    if let Ok(handlers) = get_handlers() { match handlers.add_whitelist {
        Some(add_whitelist) => match (add_whitelist)(name, uuid) {
            Ok(_) => {}
            Err(err) => warn(format!("Error adding to whitelist: {}", err)),
        },
        None => {
            let msg = Message {
                event: ERROR,
                data: MessageData {
                    error: Some("No add whitelist handler implemented".to_string()),
                    ..MessageData::default()
                },
            };
            match send(msg) {
                Ok(_) => {}
                Err(err) => warn(format!(r#"Error sending: "No add whitelist handler implemented" : {}"#, err)),
            };
        }
    } }
}

pub(crate) fn whitelist_remove(message: Message) {
    let name= match get_player(message) {
        Ok(name) => name,
        Err(_) => return,
    };

    if let Ok(handlers) = get_handlers() { match handlers.remove_whitelist {
        Some(remove_whitelist) => match (remove_whitelist)(name) {
            Ok(_) => {}
            Err(err) => warn(format!("Error removing from whitelist: {}", err)),
        },
        None => {
            let msg = Message {
                event: ERROR,
                data: MessageData {
                    error: Some("No remove whitelist handler implemented".to_string()),
                    ..MessageData::default()
                },
            };
            match send(msg) {
                Ok(_) => {}
                Err(err) => warn(format!(r#"Error sending: "No remove whitelist handler implemented" : {}"#, err)),
            };
        }
    } }
}

pub(crate) fn whitelisted_players() {
    if let Ok(handlers) = get_handlers() { match handlers.whitelisted_players {
        Some(get_whitelisted_players) => match (get_whitelisted_players)() {
            Ok(players) => {
                let msg = Message {
                    event: PLAYERS,
                    data: MessageData {
                        players: Some(players),
                        ..MessageData::default()
                    },
                };

                match send(msg) {
                    Ok(_) => {}
                    Err(err) => warn(format!("Error sending whitelisted players: {}", err)),
                };
            }
            Err(err) => {
                let msg = Message {
                    event: ERROR,
                    data: MessageData {
                        error: Some(format!("Error getting whitelisted players: {}", err)),
                        ..MessageData::default()
                    },
                };
                match send(msg) {
                    Ok(_) => {}
                    Err(err) => warn(format!(r#"Error sending: "Error getting whitelisted players": {}"#, err)),
                };
            }
        },
        None => {
            let msg = Message {
                event: ERROR,
                data: MessageData {
                    error: Some("No get whitelisted players handler implemented".to_string()),
                    ..MessageData::default()
                },
            };
            match send(msg) {
                Ok(_) => {}
                Err(err) => warn(format!(r#"Error sending: "No get whitelisted players handler implemented" : {}"#, err)),
            };
        }
    } }
}


pub(crate) fn auth_success(){
    unsafe {
        AUTHENTICATED = true;
    }
}


pub(crate) fn auth_failed(){
    unsafe {
        AUTHENTICATED = false;
    }
}