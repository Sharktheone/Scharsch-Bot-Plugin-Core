use crate::events::message::{CHAT_MESSAGE, Message, MessageData, PLAYER_ADVANCEMENT, PLAYER_JOINED, PLAYER_LEFT};
use crate::plugin::logger::{warn_no_env};
use crate::websocket::websocket::send;

pub fn player_join(name: String, server: String) {
    let msg = Message {
        event: PLAYER_JOINED,
        data: MessageData {
            player: Some(name),
            server: Some(server),
            ..MessageData::default()
        },
    };
    match send(msg){
        Ok(_) => {},
        Err(err) => warn_no_env(format!("Error sending player join message: {}", err)),
    };
}

pub fn player_leave(name: String, server: String) {
    let msg = Message {
        event: PLAYER_LEFT,
        data: MessageData {
            player: Some(name),
            server: Some(server),
            ..MessageData::default()
        },
    };
    match send(msg){
        Ok(_) => {},
        Err(err) => warn_no_env(format!("Error sending player left message: {}", err)),
    };

}

pub fn player_chat(name: String, message: String, server: String) {
    let msg = Message {
        event: CHAT_MESSAGE,
        data: MessageData {
            player: Some(name),
            message: Some(message),
            server: Some(server),
            ..MessageData::default()
        },
    };

    match send(msg){
        Ok(_) => {},
        Err(err) => warn_no_env(format!("Error sending chat message: {}", err)),
    };
}

pub fn player_death(name: String, death_message: String, server: String) {
    let msg = Message {
        event: CHAT_MESSAGE,
        data: MessageData {
            player: Some(name),
            death_message: Some(death_message),
            server: Some(server),
            ..MessageData::default()
        },
    };

    match send(msg){
        Ok(_) => {},
        Err(err) => warn_no_env(format!("Error sending death message: {}", err)),
    };
}

pub fn player_advancement(name: String, advancement: String, server: String) {
    let msg = Message {
        event: PLAYER_ADVANCEMENT,
        data: MessageData {
            player: Some(name),
            advancement: Some(advancement),
            server: Some(server),
            ..MessageData::default()
        },
    };

    match send(msg){
        Ok(_) => {},
        Err(err) => warn_no_env(format!("Error sending advancement: {}", err)),
    };
}
