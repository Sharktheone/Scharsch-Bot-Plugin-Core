use jni::objects::JObject;
use crate::plugin::kyori_adventure::component::basic_component;
use crate::plugin::kyori_adventure::parse::parse;

pub fn parse_component<'a>(data: String, is_component: bool) -> Result<JObject<'a>, ()> {
    if is_component {
        parse(data)
    } else {
        basic_component(data)
    }
}