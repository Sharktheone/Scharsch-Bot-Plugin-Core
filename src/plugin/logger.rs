use jni::JNIEnv;
use jni::objects::{JObject};
use colored::Colorize;
use chrono::{DateTime, Local};
use crate::jni_utils::{get_env_class};


pub struct Logger<'a> {
    info: &'a dyn Fn(&str, &mut JNIEnv, &JObject) -> Result<(), String>,
    warn: &'a dyn Fn(&str, &mut JNIEnv, &JObject) -> Result<(), String>,
    error: &'a dyn Fn(&str, &mut JNIEnv, &JObject) -> Result<(), String>
}

static mut LOGGER: Option<Logger> = None;

pub fn set_loggers(info: &'static dyn Fn(&str, &mut JNIEnv, &JObject) -> Result<(), String>, warn: &'static dyn Fn(&str, &mut JNIEnv, &JObject) -> Result<(), String>, error: &'static dyn Fn(&str, &mut JNIEnv, &JObject) -> Result<(), String>) {
    let logger: Logger = Logger {
        info,
        warn,
        error,
    };

    unsafe {
        LOGGER = Some(logger);
    }
}

fn time() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%H:%M:%S").to_string()
}

pub fn info<'a>(msg: String) {
    let (env, class) = match get_env_class() {
        Ok((env, class)) => (env, class),
        Err(_) => return,
    };
    unsafe {
        match &LOGGER {
            Some(logger) => {
                match (logger.info)(&*msg, env, class) {
                    Ok(_) => (),
                    Err(err) => {
                        error_no_env(format!("Error logging warn: {}", err));
                        info_no_env(msg);
                    },
                };
            }
            None => {
                info_no_env(msg);
            },
        }
    }
}

pub fn warn<'a>(msg: String) {
    let (env, class) = match get_env_class() {
        Ok((env, class)) => (env, class),
        Err(_) => return,
    };

    unsafe {
        match &LOGGER {
            Some(logger) => {
                match (logger.warn)(&*msg, env, class) {
                    Ok(_) => (),
                    Err(err) => {
                        error_no_env(format!("Error logging warn: {}", err));
                        warn_no_env(msg);
                    },
                };
            }
            None => {
                warn_no_env(msg);
            },
        }
    }
}

pub fn error<'a>(msg: String) {
    let (env, class) = match get_env_class() {
        Ok((env, class)) => (env, class),
        Err(_) => return,
    };

    unsafe {
        match &LOGGER {
            Some(logger) => {
                match (logger.error)(&*msg, env, class) {
                    Ok(_) => (),
                    Err(err) => {
                        error_no_env(format!("Error logging warn: {}", err));
                        error_no_env(msg);
                    },
                };
            }
            None => {
                error_no_env(msg);
            },
        }
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