use serde::{Deserialize, Serialize};
use crate::plugin::kyori_adventure::events::hover_event::HoverEvent;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub text: String,
    pub color: Option<String>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub insertion: Option<String>,
    // pub click_event: Option<ClickEvent>,
    pub hover_event: Option<HoverEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentNoEvents {
    pub text: String,
    pub color: Option<String>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub insertion: Option<String>,
}