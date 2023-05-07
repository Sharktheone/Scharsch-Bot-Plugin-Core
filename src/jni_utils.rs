use jni::objects::{JClass, JObject, JString, JValue};
use jni::JNIEnv;
use crate::plugin::logger::{error, error_no_env};
use jni::JavaVM;


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
#[derive(Debug, Clone, Copy)]
pub struct JniFn<'a> {
    pub name: &'a str,
    pub input: &'a [&'a str],
    pub output: &'a str,
    pub args: &'a [JValue<'a, 'a>],
}

pub static mut VM: Option<JavaVM> = None;
pub static mut CLASS: Option<JClass<'static>> = None;

pub fn set_vm(vm: JavaVM) {
    unsafe {
        VM = Some(vm);
    }
}

pub fn set_class(class: JClass<'static>) {
    unsafe {
        CLASS = Some(class);
    }
}

pub fn get_vm<'a>() -> Result<&'a mut JavaVM, ()> {
    unsafe {
        match VM.as_mut() {
            Some(vm) => Ok(vm),
            None => {
                error_no_env("No vm set!".to_string());
                Err(())
            }
        }
    }
}

pub fn get_class() -> Result<&'static JClass<'static>, ()> {
    unsafe {
        match CLASS.as_ref() {
            Some(class) => Ok(class),
            None => {
                error_no_env("No class set!".to_string());
                Err(())
            }
        }
    }
}


pub fn assemble_signature(input: &[&str], output: &str) -> String {
    let mut signature = String::from("(");
    for i in input {
        signature.push_str(make_signature(i).as_str());
    }
    signature.push(')');
    signature.push_str(make_signature(output).as_str());
    signature
}

pub fn make_signature(sig: &str) -> String {
    let mut sig = sig.replace('.', "/");

    if sig.contains('/') {
        if !sig.starts_with('L') {
            sig = format!("L{}", sig);
        }
        if !sig.ends_with(';') {
            sig = format!("{};", sig);
        }
    }

    sig
}

pub fn call_stacking<'a>(obj: &JObject<'a>, jfn: &[JniFn<'a>]) -> JObject<'a> {
    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => return JObject::null(),
    };
    let mut obj = unsafe { JObject::from_raw(obj.as_raw()) };

    for f in jfn {
        let signature = assemble_signature(f.input, f.output);
        obj = match env.call_method(obj, f.name, signature, f.args) {
            Ok(name) => {
                if f.output == JVOID {
                    return JObject::null();
                }
                match name.l() {
                    Ok(name) => name,
                    Err(e) => {
                        error(format!("Error converting jni method output {}: {}", f.name, e));
                        return JObject::null();
                    }
                }
            },
            Err(e) => {
                error(format!("Error calling jni method {}: {}", f.name, e));
                return JObject::null();
            }
        };
    }
    return unsafe { JObject::from_raw(obj.as_raw()) };
}

pub fn call_static<'a>(class: &JClass<'a>, jfn: JniFn<'a>) -> JObject<'a> {
    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => return JObject::null(),
    };

    let signature = assemble_signature(jfn.input, jfn.output);

    match env.call_static_method(class, jfn.name, signature, jfn.args) {
        Ok(name) => match name.l() {
            Ok(name) => name,
            Err(e) => {
                error(format!("Error converting jni method output {}: {}", jfn.name, e));
                return JObject::null();
            }
        },
        Err(e) => {
            error(format!("Error calling jni method {}: {}", jfn.name, e));
            return JObject::null();
        }
    }
}

pub fn call_static_stacking<'a>(class: &JClass<'a>, jfn: &[JniFn<'a>]) -> JObject<'a> {
    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => return JObject::null(),
    };


    let f = match jfn.get(0) {
        Some(f) => f,
        None => return JObject::null(),
    };

    let signature = assemble_signature(f.input, f.output);

    let obj = match env.call_static_method(class, f.name, signature, f.args) {
        Ok(name) => {
            if f.output == JVOID {
                return JObject::null();
            }
            match name.l() {
                Ok(name) => name,
                Err(e) => {
                    error(format!("Error converting static jni method output {}: {}", f.name, e));
                    return JObject::null();
                }
            }
        }
        Err(e) => {
            error(format!("Error calling static jni method {}: {}", f.name, e));
            return JObject::null();
        }
    };

    let jfn = &jfn[1..];
    call_stacking(&obj, jfn)
}


pub fn convert_string(obj: &JObject) -> String {
    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => return String::from(""),
    };
    match env.get_string(<&JString>::from(obj)) {
        Ok(s) => s.into(),
        Err(e) => {
            error(format!("Error getting string: {}", e));
            String::from("")
        }
    }
}

pub fn convert_string_or<S: Into<String>>(obj: &JObject, default: S) -> String {
    let mut env = match get_env() {
        Ok(env) => env,
        Err(_) => return String::from(""),
    };
    match env.get_string(<&JString>::from(obj)) {
        Ok(s) => s.into(),
        Err(e) => {
            error(format!("Error getting string: {}", e));
            default.into()
        }
    }
}


pub fn get_env<'a>() -> Result<JNIEnv<'a>, ()> {
    let vm = match get_vm() {
        Ok(vm) => vm,
        Err(_) => {
            return Err(());
        }
    };

    match vm.get_env() {
        Ok(env) => {
            match env.exception_describe() {
                Ok(_) => {}
                Err(err) => {
                    error(format!("Error enabling descriptions on exceptions: {}", err));
                }
            };
            Ok(env)
        },
        Err(err) => {
            error_no_env(format!("Failed getting env: {}", err));
            Err(())
        }
    }
}