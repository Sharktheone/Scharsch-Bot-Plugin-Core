use jni::JNIEnv;
use jni::objects::JObject;
use crate::events::message::{CHAT_MESSAGE, Message, MessageData, PLAYER_JOINED, PLAYER_LEFT};
use crate::plugin::logger::{warn};
use crate::websocket::websocket::send;

pub fn player_join(env: &mut JNIEnv, class: &JObject, name: String, server: String) {
    let msg = Message {
        event: PLAYER_JOINED,
        data: MessageData {
            player: Some(name),
            server: Some(server),
            ..MessageData::default()
        },
    };
    match send(env, class, msg){
        Ok(_) => {},
        Err(err) => warn(env, class, format!("Error sending player join message: {}", err)),
    };
}

pub fn player_leave(env: &mut JNIEnv, class: &JObject, name: String, server: String) {
    let msg = Message {
        event: PLAYER_LEFT,
        data: MessageData {
            player: Some(name),
            server: Some(server),
            ..MessageData::default()
        },
    };
    match send(env, class, msg){
        Ok(_) => {},
        Err(err) => warn(env, class, format!("Error sending player left message: {}", err)),
    };

}

pub fn player_chat(env: &mut JNIEnv, class: &JObject, name: String, message: String, server: String) {
    let msg = Message {
        event: CHAT_MESSAGE,
        data: MessageData {
            player: Some(name),
            message: Some(message),
            server: Some(server),
            ..MessageData::default()
        },
    };

    match send(env, class, msg){
        Ok(_) => {},
        Err(err) => warn(env, class, format!("Error sending chat message: {}", err)),
    };
}