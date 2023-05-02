use jni::objects::JObject;
use crate::jni_utils::get_env;

pub fn parse<'a, S: Into<String>>(msg: S) -> JObject<'a> {
    let msg = msg.into();


    // use gson parser to deserialize the message


    JObject::null()
}