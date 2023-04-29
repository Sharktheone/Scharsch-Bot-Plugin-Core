use std::ops::Deref;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::JNIEnv;
use crate::{CLASS};
use crate::plugin::logger::{error_no_env};


#[allow(unused)]
pub const JVOID: &str = "V";

#[allow(unused)]
pub const JBOOLEAN: &str = "Z";

#[allow(unused)]
pub const JBYTE: &str = "B";

#[allow(unused)]
pub const JCHAR: &str = "C";

#[allow(unused)]
pub const JSHORT: &str = "S";

#[allow(unused)]
pub const JINT: &str = "I";

#[allow(unused)]
pub const JLONG: &str = "J";

#[allow(unused)]
pub const JFLOAT: &str = "F";

#[allow(unused)]
pub const JDOUBLE: &str = "D";

#[allow(unused)]
pub const JSTRING: &str = "Ljava/lang/String;";

#[allow(unused)]

pub struct JniFn<'a> {
    pub name: &'a str,
    pub input: &'a [&'a str],
    pub output: &'a str,
    pub args: &'a [JValue<'a, 'a>],
}

pub fn assemble_signature(input: &[&str], output: &str) -> String {
    let mut signature = String::from("(");
    for i in input {
        signature.push_str(make_signature(&i.to_string()).as_str());
    }
    signature.push_str(")");
    signature.push_str(make_signature(&output.to_string()).as_str());
    return signature;
}

pub fn make_signature(sig: &String) -> String {
    let mut sig = sig.replace(".", "/");

    if sig.contains("/") {
        if !sig.starts_with("L") {
            sig = format!("L{}", sig);
        }
        if !sig.ends_with(";") {
            sig = format!("{};", sig);
        }
    }

    return sig
}

pub fn call_stacking<'a, 'b>(env: &mut JNIEnv, obj: &JObject<'b>, jfn: &[JniFn<'a>]) -> JObject<'a> {
    let mut obj = unsafe { JObject::from_raw(obj.as_raw()) };

    for f in jfn {
        let signature = assemble_signature(f.input, &f.output);
        obj = match env.call_method(obj, &f.name, signature, f.args) {
            Ok(name) => name.l().unwrap(),
            Err(e) => {
                error_no_env(format!("Error calling jni method {}: {}", f.name, e));
                return JObject::null();
            }
        };
    }
    return unsafe { JObject::from_raw(obj.as_raw()) }
}


pub fn convert_string(env: &mut JNIEnv, obj: &JObject) -> String {
    match env.get_string(<&JString>::from(obj)) {
        Ok(s) => s.into(),
        Err(e) => {
            error_no_env(format!("Error getting string: {}", e));
            return String::from("");
        }
    }
}

// pub(crate) fn get_env() -> Result<&'static mut JNIEnv<'static>, ()> {
//     unsafe {
//         match ENV.as_mut() {
//             Some(env) => Ok(env),
//             None => {
//                 error_no_env("No env set!".to_string());
//                 Err(())
//             }
//         }
//     }
// }

pub(crate) fn get_class() -> Result<&'static mut JClass<'static>, ()> {
    unsafe {
        match CLASS.as_mut() {
            Some(class) => Ok(class),
            None => {
                error_no_env("No class set!".to_string());
                Err(())
            }
        }
    }
}

// pub(crate) fn get_env_class() -> Result<(&'static mut JNIEnv<'static>, &'static mut JClass<'static>), ()> {
//     unsafe {
//         match (ENV.as_mut(), CLASS.as_mut()) {
//             (Some(env), Some(class)) => Ok((env, class)),
//             _ => {
//                 error_no_env("No env or class set!".to_string());
//                 Err(())
//             }
//         }
//     }
// }