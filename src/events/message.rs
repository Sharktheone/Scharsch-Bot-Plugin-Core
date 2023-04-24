use serde::{Deserialize, Serialize};



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