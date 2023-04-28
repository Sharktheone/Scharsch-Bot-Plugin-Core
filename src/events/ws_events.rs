// Bot => Server: SendPlayers
// Bot => Server: KickPlayer
// Bot => Server: ReportPlayer
// Bot => Server: BanPlayer
// Bot => Server: UnbanPlayer
// Bot => Server: SendCommand
// Bot => Server: SendChatMessage
// Bot => Server: AuthSuccess
// Bot => Server: AuthFailed
// Bot => Server: Error
// Bot => Server: WhitelistPlayer
// Bot => Server: UnwhitelistPlayer

use jni::JNIEnv;
use jni::objects::JClass;
use crate::events::handler::{HANDLERS, Handlers};
use crate::events::message::{ERROR, Message, MessageData, PLAYERS};
use crate::plugin::logger::warn;
use crate::websocket::websocket::send;

fn get_handlers(env: &mut JNIEnv, class: &JClass) -> Result<&'static Handlers, ()> {
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
                match send(env, class, msg) {
                    Ok(_) => {},
                    Err(err) => {warn(env, class, format!(r#"Error sending: "No handlers implemented" : {}"#, err)) },
                };
                Err(())
            }
        }
    }
}

pub(crate) fn send_players(env: &mut JNIEnv, class: &JClass) {
    match get_handlers(env, class) {
        Ok(handlers) => match handlers.get_players_handler {
            Some(get_players_handler) => match (get_players_handler)() {
                Ok(players) => {
                    let msg = Message {
                        event: PLAYERS,
                        data: MessageData {
                            players: Some(players),
                            ..MessageData::default()
                        },
                    };

                    match send(env, class, msg) {
                        Ok(_) => {}
                        Err(err) => warn(env, class, format!("Error sending players: {}", err)),
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
                    match send(env, class, msg) {
                        Ok(_) => {}
                        Err(err) => warn(env, class, format!(r#"Error sending: "Error getting players": {}"#, err)),
                    };
                    return;
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
                match send(env, class, msg) {
                    Ok(_) => {}
                    Err(err) => warn(env, class, format!(r#"Error sending: "No get players handler implemented" : {}"#, err)),
                };
                return;
            }
        }
        Err(_) => return,
    };
}