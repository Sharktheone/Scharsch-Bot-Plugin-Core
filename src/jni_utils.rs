use jni::objects::{JClass, JObject, JString, JValue};
use jni::JNIEnv;
use crate::plugin::logger::{error_no_env};
use crate::{get_vm};


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

    return sig;
}

pub fn call_stacking<'a, 'b>(obj: &JObject<'b>, jfn: &[JniFn<'a>]) -> JObject<'a> {
    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => return JObject::null(),
    };
    let mut obj = unsafe { JObject::from_raw(obj.as_raw()) };

    for f in jfn {
        let signature = assemble_signature(f.input, &f.output);
        obj = match env.call_method(obj, &f.name, signature, f.args) {
            Ok(name) => match name.l(){
                Ok(name) => name,
                Err(e) => {
                    error_no_env(format!("Error calling jni method {}: {}", f.name, e));
                    return JObject::null();
                }
            },
            Err(e) => {
                error_no_env(format!("Error calling jni method {}: {}", f.name, e));
                return JObject::null();
            }
        };
    }
    return unsafe { JObject::from_raw(obj.as_raw()) };
}


pub fn convert_string(obj: &JObject) -> String {
    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => return String::from(""),
    };
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

pub fn get_env<'a>() -> Result<JNIEnv<'a>, ()> {
    let vm = match get_vm() {
        Ok(vm) => vm,
        Err(_) => {
            error_no_env(format!("No vm set!"));
            return Err(());
        }
    };

    match vm.get_env() {
        Ok(env) => Ok(env),
        Err(err) => {
            error_no_env(format!("Failed getting env: {}", err));
            return Err(());
        }
    }
}

pub fn get_class<'a>() -> Result<JClass<'a>, ()> {
    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => {
            error_no_env(format!("No env set!"));
            return Err(());
        }
    };
    match env.find_class("de/scharschbot/velocity/plugin/Events") {
        Ok(class) => Ok(class),
        Err(err) => {
            error_no_env(format!("Error getting class: {}", err));
            return Err(());
        }
    }
}

pub fn get_env_class<'a>() -> Result<(JNIEnv<'a>, JClass<'a>), ()> {
    let env = match get_env() {
        Ok(env) => env,
        Err(_) => {
            error_no_env(format!("No env set!"));
            return Err(());
        }
    };
    let class = match get_class() {
        Ok(class) => class,
        Err(_) => {
            error_no_env(format!("No class set!"));
            return Err(());
        }
    };
    Ok((env, class))
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