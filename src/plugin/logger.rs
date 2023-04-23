use jni::JNIEnv;
use jni::objects::{JObject, JValue};

pub struct Logger {
    info: fn(&str),
    warn: fn(&str),
    error: fn(&str),
}

pub fn set_loggers(info: fn(&str), warn: fn(&str), error: fn(&str), env: &mut JNIEnv<'_>, class: &JObject) {
    let logger: Logger = Logger {
        info,
        warn,
        error,
    };

    let logger_ptr: *const Logger = &logger;
    let ptr = logger_ptr as i64;

    match env.set_field(class, "logger", "J", JValue::Long(ptr)) {
        Ok(_) => (),
        Err(err) => eprintln!("Error setting logger: {}", err),
    }
}

fn get_loggers<'a>(env: &mut JNIEnv<'a>, class: &JObject) -> Result<*const Logger, String> {
    match env.get_field(class, "logger", "J") {
        Ok(ptr_val) => {
            match ptr_val.j() {
                Ok(ptr_j) => {
                    if ptr_j == 0 {
                        Err("No logger pointer stored".to_string())
                    } else {
                        let ptr: *const Logger = ptr_j as *const Logger;
                        Ok(ptr as *const Logger)
                    }
                }
                Err(err) => Err(format!("Error getting logger pointer: {}", err)),
            }
        }
        Err(err) => Err(format!("Error getting logger pointer: {}", err)),
    }
}

pub fn info<'a>(env: &mut JNIEnv<'a>, class: &JObject, msg: &str) {
    match get_loggers(env, class) {
        Ok(logger_ptr) => {
            let logger: &Logger = unsafe { &*logger_ptr };
            (logger.info)(msg);
        }
        Err(err) => eprintln!("Error getting logger: {}", err),
    }
}

pub fn warn<'a>(env: &mut JNIEnv<'a>, class: &JObject, msg: &str) {
    match get_loggers(env, class) {
        Ok(logger_ptr) => {
            let logger: &Logger = unsafe { &*logger_ptr };
            (logger.warn)(msg);
        }
        Err(err) => eprintln!("Error getting logger: {}", err),
    }
}

pub fn error<'a>(env: &mut JNIEnv<'a>, class: &JObject, msg: &str) {
    match get_loggers(env, class) {
        Ok(logger_ptr) => {
            let logger: &Logger = unsafe { &*logger_ptr };
            (logger.error)(msg);
        }
        Err(err) => eprintln!("Error getting logger: {}", err),
    }
}