use serde::{Deserialize, Serialize};


const SEND_PLAYERS: &str = "sendPlayers";
const KICK_PLAYER: &str = "kickPlayer";
const REPORT_PLAYER: &str = "reportPlayer";
const BAN_PLAYER: &str = "banPlayer";
const UNBAN_PLAYER: &str = "unbanPlayer";
const SEND_COMMAND: &str = "sendCommand";
const SEND_CHAT_MESSAGE: &str = "sendChatMessage";
const PLAYER_JOINED: &str = "playerJoined";
const PLAYER_LEFT: &str = "playerLeft";
const PLAYERS: &str = "players";
const CHAT_MESSAGE: &str = "chatMessage";
const PLAYER_DEATH: &str = "playerDeath";
const PLAYER_ADVANCEMENT: &str = "playerAdvancement";
const AUTH: &str = "auth";
const AUTH_SUCCESS: &str = "authSuccess";
const AUTH_FAILED: &str = "authFailed";
const ERROR: &str = "error";


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