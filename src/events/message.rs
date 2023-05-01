use serde::{Deserialize, Serialize};

pub const SEND_PLAYERS: &str = "sendPlayers";
pub const KICK_PLAYER: &str = "kickPlayer";
pub const REPORT_PLAYER: &str = "reportPlayer";
pub const BAN_PLAYER: &str = "banPlayer";
pub const UNBAN_PLAYER: &str = "unbanPlayer";
pub const SEND_COMMAND: &str = "sendCommand";
pub const SEND_CHAT_MESSAGE: &str = "sendChatMessage";
pub const WHITELIST_ADD: &str = "whitelistAdd";
pub const WHITELIST_REMOVE: &str = "whitelistRemove";
pub const WHITELISTED_PLAYERS: &str = "whitelistedPlayers";
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


#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Message<'a> {
    pub event: &'a str,
    pub data: MessageData,

}

#[derive(Debug, Deserialize, Serialize, Default)]
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
    pub server: Option<String>,
}
