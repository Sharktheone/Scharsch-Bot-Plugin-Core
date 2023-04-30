use colored::Colorize;
use chrono::{DateTime, Local};

pub struct Logger<'a> {
    info: &'a dyn Fn(&str) -> Result<(), String>,
    warn: &'a dyn Fn(&str) -> Result<(), String>,
    error: &'a dyn Fn(&str) -> Result<(), String>
}

static mut LOGGER: Option<Logger> = None;

pub fn set_loggers(info: &'static dyn Fn(&str) -> Result<(), String>, warn: &'static dyn Fn(&str) -> Result<(), String>, error: &'static dyn Fn(&str) -> Result<(), String>) {
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

pub fn info<S: Into<String>>(msg: S) {
    let msg: String = msg.into();
    unsafe {
        match &LOGGER {
            Some(logger) => {
                match (logger.info)(&*msg) {
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

pub fn warn<S: Into<String>>(msg: S) {
    let msg: String = msg.into();
    unsafe {
        match &LOGGER {
            Some(logger) => {
                match (logger.warn)(&*msg) {
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

pub fn error<S: Into<String>>(msg: S) {
    let msg: String = msg.into();
    unsafe {
        match &LOGGER {
            Some(logger) => {
                match (logger.error)(&*msg) {
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
    println!("{}", format!("[{} WARN] [ScharschBot/core]: {}", time(), msg).yellow());
}

pub fn error_no_env(msg: String) {
    println!("{}", format!("[{} ERROR] [ScharschBot/core]: {}", time(), msg).red());
}