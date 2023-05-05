use jni::objects::JObject;

pub fn parse<'a, S: Into<String>>(msg: S) -> JObject<'a> {
    let _msg = msg.into();


    // use gson parser to deserialize the message


    JObject::null()
}