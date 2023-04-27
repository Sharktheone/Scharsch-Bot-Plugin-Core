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
use crate::events::handler::HANDLERS;
use crate::events::message::{ERROR, Message, MessageData, PLAYERS};
use crate::plugin::logger::warn;
use crate::websocket::websocket::send;

pub(crate) fn send_players(env: &mut JNIEnv, class: &JClass) {
   unsafe {
        match HANDLERS.as_ref() {
            Some(players) => match (players.get_players_handler)() {
                Ok(players) => {
                    let msg = Message {
                        event: PLAYERS,
                        data: MessageData {
                            players: Some(players),
                            ..MessageData::default()
                        },
                    };

                    match send(env, class, msg) {
                        Ok(_) => {},
                        Err(err) => warn(env, class, format!("Error sending players: {}", err)),
                    };
                },
                Err(err) => {
                    let msg = Message {
                        event: ERROR,
                        data: MessageData {
                            error: Some(format!("Error getting players: {}", err)),
                            ..MessageData::default()
                        },
                    };
                    match send(env, class, msg) {
                        Ok(_) => {},
                        Err(err) => warn(env, class, format!(r#"Error sending: "Error getting players": {}"#, err)),
                    };
                    return;
                }
            },
            None => {
                let msg = Message {
                    event: ERROR,
                    data: MessageData {
                        error: Some("No get_players_handler set".to_string()),
                        ..MessageData::default()
                    },
                };
                match send(env, class, msg) {
                    Ok(_) => {},
                    Err(err) => warn(env, class, format!(r#"Error sending: "No get_players_handler set" : {}"#, err)),
                };
                return;
            }
        }
    };
}