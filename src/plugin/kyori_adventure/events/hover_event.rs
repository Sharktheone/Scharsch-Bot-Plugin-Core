use crate::plugin::kyori_adventure::component::Component;

pub struct HoverEvent {
    pub action: String,
    pub show_text: Option<ShowText>,
    pub show_item: Option<ShowItem>,
    pub show_entity: Option<ShowEntity>,
}

pub struct ShowText {
    pub text: Component,

}

pub struct ShowItem {
    pub item: String,
    pub count: Option<i32>,
}

pub struct ShowEntity {
    pub entity: String,
    pub uuid: String,
    pub name: Option<Component>,
}