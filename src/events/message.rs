use serde::{Deserialize, Serialize};

pub const SEND_PLAYERS: String = String::from("sendPlayers");
pub const KICK_PLAYER: String = String::from("kickPlayer");
pub const REPORT_PLAYER: String = String::from("reportPlayer");
pub const BAN_PLAYER: String = String::from("banPlayer");
pub const UNBAN_PLAYER: String = String::from("unbanPlayer");
pub const SEND_COMMAND: String = String::from("sendCommand");
pub const SEND_CHAT_MESSAGE: String = String::from("sendChatMessage");
pub const PLAYER_JOINED: String = String::from("playerJoined");
pub const PLAYER_LEFT: String = String::from("playerLeft");
pub const PLAYERS: String = String::from("players");
pub const CHAT_MESSAGE: String = String::from("chatMessage");
pub const PLAYER_DEATH: String = String::from("playerDeath");
pub const PLAYER_ADVANCEMENT: String = String::from("playerAdvancement");
pub const AUTH: String = String::from("auth");
pub const AUTH_SUCCESS: String = String::from("authSuccess");
pub const AUTH_FAILED: String = String::from("authFailed");
pub const ERROR: String = String::from("error");


#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub event: String,
    #[serde(flatten)]
    pub data: MessageData,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageData {
    pub players: Option<Vec<String>>,
    pub player: Option<String>,
    pub reason: Option<String>,
    pub command: Option<String>,
    pub message: Option<String>,
    pub death_message: Option<String>,
    pub advancement: Option<String>,
    pub password: Option<String>,
    pub user: Option<String>,
    pub error: Option<String>,
}