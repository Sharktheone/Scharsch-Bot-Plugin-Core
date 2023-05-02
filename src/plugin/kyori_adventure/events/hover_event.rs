use crate::plugin::kyori_adventure::component::ComponentNoEvents;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverEvent {
    pub action: String,
    pub show_text: Option<ShowText>,
    pub show_item: Option<ShowItem>,
    pub show_entity: Option<ShowEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowText {
    pub text: ComponentNoEvents,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowItem {
    pub item: String,
    pub count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowEntity {
    pub entity: String,
    pub uuid: String,
    pub name: Option<ComponentNoEvents>,
}