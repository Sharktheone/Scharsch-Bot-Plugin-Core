use jni::objects::{JObject, JString};
use crate::jni_utils::get_env;
use crate::plugin::kyori_adventure::component::basic_component;
use crate::plugin::kyori_adventure::legacy_serialize::legacy_serialize;
use crate::plugin::kyori_adventure::parse::parse;
use crate::plugin::logger::error;

pub fn parse_component<'a>(data: String, is_component: bool) -> Result<JObject<'a>, ()> {
    if is_component {
        parse(data)
    } else {
        basic_component(data)
    }
}

pub fn parse_component_to_legacy<'a>(data: String, is_component: bool) -> Result<JString<'a>, ()> {
   if is_component {
       let component = match parse(data) {
           Ok(component) => component,
           Err(_) => return Err(()),
       };

       legacy_serialize(component)
   } else {
       let mut env = match get_env() {
           Ok(env) => env,
           Err(_) => return Err(()),
       };

       let data_string = match env.new_string(data) {
              Ok(data_string) => data_string,
              Err(err) => {
                error(format!("Error creating msg string: {}", err));
                return Err(());
              },
       };
         Ok(data_string)
   }
}