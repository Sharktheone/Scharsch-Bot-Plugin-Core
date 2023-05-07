use jni::objects::{JObject, JString, JValue};
use crate::jni_utils::{call_stacking, get_env, JniFn, JSTRING};
use crate::plugin::logger::error;

pub fn legacy_serialize(component: JObject) -> Result<JString, ()> {
let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => return Err(()),
    };

    let serializer = match env.find_class("net/kyori/adventure/text/serializer/legacy/LegacyComponentSerializer"){
        Ok(serializer) => serializer,
        Err(err) => {
            error(format!("Error creating serializer: {}", err));
            return Err(());
        },
    };

    let arg: JValue = JValue::Object(&component);

    let fns = [
        JniFn {
            name: "legacySection",
            input: &[],
            output: "net.kyori.adventure.text.serializer.legacy.LegacyComponentSerializer",
            args: &[],
        },
        JniFn {
            name: "serialize",
            input: &["net.kyori.adventure.text.Component"],
            output: JSTRING,
            args: &[arg],
        },
    ];

    let final_component = call_stacking(&serializer, &fns);

    Ok(JString::from(final_component))
}