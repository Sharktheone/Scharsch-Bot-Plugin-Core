use jni::objects::{JObject, JValue};
use crate::jni_utils::{call_stacking, get_env, JniFn, JSTRING};
use crate::plugin::logger::error;

pub fn parse<'a, S: Into<String>>(component: S) -> Result<JObject<'a>, ()> {
    let component = component.into();

    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => {
            error("Error getting env");
            return Err(());
        },
    };

    let serializer = match env.find_class("net/kyori/adventure.text/serializer/gson/GsonComponentSerializer"){
        Ok(serializer) => serializer,
        Err(err) => {
            error(format!("Error creating serializer: {}", err));
            return Err(());
        },
    };

    let msg_string = match env.new_string(component) {
        Ok(msg_string) => msg_string,
        Err(err) => {
            error(format!("Error creating msg string: {}", err));
            return Err(());
        },
    };

    let msg_obj = JObject::from(msg_string);
    let arg: JValue = JValue::Object(&msg_obj);

    let fns = [
        JniFn {
            name: "gson",
            input: &[],
            output: "net.kyori.adventure.text.serializer.gson.GsonComponentSerializer",
            args: &[],
        },
        JniFn {
            name: "deserialize",
            input: &[JSTRING],
            output: "net.kyori.adventure.text.Component",
            args: &[arg],
        },
    ];

    let final_component = call_stacking(&serializer, &fns);

    unsafe {
        Ok(JObject::from_raw(final_component.as_raw()))
    }
}