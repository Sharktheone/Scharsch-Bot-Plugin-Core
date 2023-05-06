use jni::objects::{JObject, JValue};
use crate::jni_utils::{call_static, get_env, JniFn, JSTRING};
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
            error(format!("Error getting component: {}", err));
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
    let fns = JniFn {
            name: "text",
            input: &[JSTRING],
            output: "Lnet/kyori/adventure/text/TextComponent;",
            args: &[arg],
        };

    let final_component = call_static(&component, fns);

    unsafe {
        Ok(JObject::from_raw(final_component.as_raw()))
    }
}