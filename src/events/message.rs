use serde::{Deserialize, Serialize};


pub const SEND_PLAYERS: &str = "sendPlayers";
pub const KICK_PLAYER: &str = "kickPlayer";
pub const REPORT_PLAYER: &str = "reportPlayer";
pub const BAN_PLAYER: &str = "banPlayer";
pub const UNBAN_PLAYER: &str = "unbanPlayer";
pub const SEND_COMMAND: &str = "sendCommand";
pub const SEND_CHAT_MESSAGE: &str = "sendChatMessage";
pub const PLAYER_JOINED: &str = "playerJoined";
pub const PLAYER_LEFT: &str = "playerLeft";
pub const PLAYERS: &str = "players";
pub const CHAT_MESSAGE: &str = "chatMessage";
pub const PLAYER_DEATH: &str = "playerDeath";
pub const PLAYER_ADVANCEMENT: &str = "playerAdvancement";
pub const AUTH: &str = "auth";
pub const AUTH_SUCCESS: &str = "authSuccess";
pub const AUTH_FAILED: &str = "authFailed";
pub const ERROR: &str = "error";


#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    event: String,
    #[serde(flatten)]
    data: MessageData,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageData {
    players: Option<Vec<String>>,
    player: Option<String>,
    reason: Option<String>,
    command: Option<String>,
    message: Option<String>,
    death_message: Option<String>,
    advancement: Option<String>,
    password: Option<String>,
    user: Option<String>,
    error: Option<String>,
}