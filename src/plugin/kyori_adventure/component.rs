use jni::objects::{JObject, JValue};
use crate::jni_utils::{call_stacking, get_env, JniFn};
use crate::plugin::logger::error;

pub fn basic_component<'a>(text: String) -> Result<JObject<'a>, ()> {
    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => {
            error("Error getting env");
            return Err(());
        },
    };

    let component = match env.find_class("net/kyori/adventure/text/Component") {
        Ok(component) => component,
        Err(err) => {
            error(format!("Error creating component: {}", err));
            return Err(());
        },
    };

    let text_string = match env.new_string(text) {
        Ok(text_string) => text_string,
        Err(err) => {
            error(format!("Error creating text string: {}", err));
            return Err(());
        },
    };
    let text_obj = JObject::from(text_string);
    let arg: JValue = JValue::Object(&text_obj);

    let fns = [
        JniFn {
            name: "text",
            input: &[],
            output: "net.kyori.adventure.text.Component",
            args: &[arg],
        }
    ];

    let final_component = call_stacking(&component, &fns);

    unsafe {
        Ok(JObject::from_raw(final_component.as_raw()))
    }
}