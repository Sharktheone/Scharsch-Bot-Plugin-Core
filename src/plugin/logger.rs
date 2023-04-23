use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use colored::Colorize;
use chrono::{DateTime, Local};


pub struct Logger {
    info: fn(&str) -> Result<(), String>,
    warn: fn(&str) -> Result<(), String>,
    error: fn(&str) -> Result<(), String>,
}

pub fn set_loggers(info: fn(&str) -> Result<(), String>, warn: fn(&str) -> Result<(), String>, error: fn(&str) -> Result<(), String>, env: &mut JNIEnv<'_>, class: &JObject) {
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

fn time() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%H:%M:%S").to_string()
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
            match (logger.info)(msg){
                Ok(_) => (),
                Err(err) => {
                    eprintln!("{}", format!("Error logging info: {}", err).red());
                    println!("[{} INFO] [ScharschBot/core] {}", time(), msg);

                },
            };
        }
        Err(err) => {
            eprintln!("{}", format!("Error getting logger: {}", err).red());
            println!("[{} INFO] [ScharschBot/core] {}", time(), msg);
        },
    }
}

pub fn warn<'a>(env: &mut JNIEnv<'a>, class: &JObject, msg: &str) {
    match get_loggers(env, class) {
        Ok(logger_ptr) => {
            let logger: &Logger = unsafe { &*logger_ptr };
            match (logger.warn)(msg){
                Ok(_) => (),
                Err(err) => {
                    eprintln!("{}", format!("Error logging warn: {}", err).red());
                    println!("{}", format!("[{} WARN] [ScharschBot/core] {}", time(), msg).yellow());
                },
            };
        }
        Err(err) => {
            eprintln!("{}", format!("Error getting logger: {}", err).red());
            println!("{}", format!("[{} WARN] [ScharschBot/core] {}", time(), msg).yellow());
        },
    }
}

pub fn error<'a>(env: &mut JNIEnv<'a>, class: &JObject, msg: &str) {
    match get_loggers(env, class) {
        Ok(logger_ptr) => {
            let logger: &Logger = unsafe { &*logger_ptr };
            match (logger.error)(msg) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("{}", format!("Error logging error: {}", err).red());
                    println!("{}", format!("[{} ERROR] [ScharschBot/core] {}", time(), msg).red());
                },
            };
        }
        Err(err) => {
            eprintln!("{}", format!("Error getting logger: {}", err).red());
            println!("{}", format!("[{} ERROR] [ScharschBot/core] {}", time(), msg).red());
        },
    }
}