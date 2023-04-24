use jni::JNIEnv;
use jni::objects::JObject;
use crate::events::message::{Message, MessageData};
use crate::websocket::websocket::send;

fn player_join(env: &mut JNIEnv, class: &JObject, name: String) {
    println!("Player joined: {}", name);
    let msg = Message {
        event:
    };
    send(env, class, "Hello from Rust!".to_string());
}

fn player_leave(env: &mut JNIEnv, class: &JObject, name: String) {
    println!("Player left: {}", name);
}

fn player_chat(env: &mut JNIEnv, class: &JObject, name: String, message: String) {
    println!("Player chat: {} - {}", name, message);
}