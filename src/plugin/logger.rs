use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};
use colored::Colorize;
use chrono::{DateTime, Local};


pub struct Logger<'a> {
    info: &'a dyn Fn(&str, &mut JNIEnv, &JObject) -> Result<(), String>,
    warn: &'a dyn Fn(&str, &mut JNIEnv, &JObject) -> Result<(), String>,
    error: &'a dyn Fn(&str, &mut JNIEnv, &JObject) -> Result<(), String>
}

pub fn set_loggers(info: &dyn Fn(&str, &mut JNIEnv, &JObject) -> Result<(), String>, warn: &dyn Fn(&str, &mut JNIEnv, &JObject) -> Result<(), String>, error: &dyn Fn(&str, &mut JNIEnv, &JObject) -> Result<(), String>, env: &mut JNIEnv<'_>, class: &JObject) {
    let logger: Logger = Logger {
        info,
        warn,
        error,
    };

    let logger_ptr: *const Logger = &logger;
    let ptr = logger_ptr as i64;

    match env.set_field(class, "logger_ptr", "J", JValue::Long(ptr)) {
        Ok(_) => (),
        Err(err) => error_no_env(format!("Error setting logger: {}", err)),
    }
}

fn time() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%H:%M:%S").to_string()
}

fn get_loggers<'a>(env: &mut JNIEnv<'a>, class: &JObject) -> Result<*const Logger<'a>, String> {
    match env.get_field(class, "logger_ptr", "J") {
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

pub fn info<'a>(env: &mut JNIEnv<'a>, class: &JObject, msg: String) {
    match get_loggers(env, class) {
        Ok(logger_ptr) => {
            let logger: &Logger = unsafe { &*logger_ptr };
            match (logger.info)(&*msg, env, class){
                Ok(_) => (),
                Err(err) => {
                    error_no_env(format!("Error logging info: {}", err));
                    info_no_env(msg);

                },
            };
        }
        Err(err) => {
            error_no_env(format!("Error getting logger: {}", err));
            info_no_env(msg);
        },
    }
}

pub fn warn<'a>(env: &mut JNIEnv<'a>, class: &JObject, msg: String) {
    match get_loggers(env, class) {
        Ok(logger_ptr) => {
            let logger: &Logger = unsafe { &*logger_ptr };
            match (logger.warn)(&*msg, env, class){
                Ok(_) => (),
                Err(err) => {
                    error_no_env(format!("Error logging warn: {}", err));
                    warn_no_env(msg);
                },
            };
        }
        Err(err) => {
            error_no_env(format!("Error getting logger: {}", err));
            warn_no_env(msg);
        },
    }
}

pub fn error<'a>(env: &mut JNIEnv<'a>, class: &JObject, msg: String) {
    match get_loggers(env, class) {
        Ok(logger_ptr) => {
            let logger: &Logger = unsafe { &*logger_ptr };
            match (logger.error)(&*msg, env, class) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("{}", format!("Error logging error: {}", err).red());
                    error_no_env(msg);
                },
            };
        }
        Err(err) => {
            eprintln!("{}", format!("Error getting logger: {}", err).red());
            error_no_env(msg);
        },
    }
}

pub fn info_no_env(msg: String) {
    println!("[{} INFO] [ScharschBot/core]: {}", time(), msg);
}

pub fn warn_no_env(msg: String) {
    println!("{}", format!("[{} WARN]: [ScharschBot/core] {}", time(), msg).yellow());
}

pub fn error_no_env(msg: String) {
    println!("{}", format!("[{} ERROR]: [ScharschBot/core] {}", time(), msg).red());
}