use std::sync::mpsc;
use colored::Colorize;
use chrono::{DateTime, Local};

pub struct Logger<'a> {
    info: &'a dyn Fn(&str) -> Result<(), String>,
    warn: &'a dyn Fn(&str) -> Result<(), String>,
    error: &'a dyn Fn(&str) -> Result<(), String>
}

static mut LOGGER: Option<Logger> = None;
static mut SENDER: Option<mpsc::Sender<(String, String)>> = None;

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
    log("info", msg);
    // unsafe {
    //     match &LOGGER {
    //         Some(logger) => {
    //             match (logger.info)(&*msg) {
    //                 Ok(_) => (),
    //                 Err(err) => {
    //                     error_no_env(format!("Error logging warn: {}", err));
    //                     info_no_env(msg);
    //                 },
    //             };
    //         }
    //         None => {
    //             error_no_env("No logger set!".to_string());
    //             info_no_env(msg);
    //         },
    //     }
    // }
}

pub fn warn<S: Into<String>>(msg: S) {
    let msg: String = msg.into();
    log("warn", msg);
    // unsafe {
    //     match &LOGGER {
    //         Some(logger) => {
    //             match (logger.warn)(&*msg) {
    //                 Ok(_) => (),
    //                 Err(err) => {
    //                     error_no_env(format!("Error logging warn: {}", err));
    //                     warn_no_env(msg);
    //                 },
    //             };
    //         }
    //         None => {
    //             error_no_env("No logger set!".to_string());
    //             warn_no_env(msg);
    //         },
    //     }
    // }
}

pub fn error<S: Into<String>>(msg: S) {
    let msg: String = msg.into();
    log("error", msg);
    // unsafe {
    //     match &LOGGER {
    //         Some(logger) => {
    //             match (logger.error)(&*msg) {
    //                 Ok(_) => (),
    //                 Err(err) => {
    //                     error_no_env(format!("Error logging warn: {}", err));
    //                     error_no_env(msg);
    //                 },
    //             };
    //         }
    //         None => {
    //             error_no_env("No logger set!".to_string());
    //             error_no_env(msg);
    //         },
    //     }
    // }
}

fn log(level: &'static str, msg: String) {
    unsafe {
        match &SENDER {
            Some(sender) => {
                match sender.send((level.to_string(), msg.clone())) {
                    Ok(_) => (),
                    Err(err) => {
                        error_no_env(format!("Error sending log message: {}", err));
                        log_no_env_level(level, msg)

                    },
                };
            }
            None => {
                error_no_env("No logger set!".to_string());
                log_no_env_level(level, msg);
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


pub fn logger_pump() {
    let (tx, rx): (mpsc::Sender<(String, String)>, mpsc::Receiver<(String, String)>) = mpsc::channel();

    unsafe {
        SENDER = Some(tx);
    }

    loop {
        let (level, msg) = rx.recv().unwrap();
        unsafe {
            match &LOGGER {
                Some(logger) => {
                    let log = match level.as_str() {
                        "info" => logger.info,
                        "warn" => logger.warn,
                        "error" => logger.error,
                        _ => {
                            error_no_env(format!("Unknown log level: {}", level));
                            log_no_env_level(level, msg);
                            return;
                        },
                    };

                    match (log)(&*msg) {
                        Ok(_) => (),
                        Err(err) => {
                            error_no_env(format!("Error logging {}: {}", level, err));
                            log_no_env_level(level, msg);
                        },
                    };
                }
                None => {
                    error_no_env("No logger set!".to_string());
                    log_no_env_level(level, msg);
                },
            }
        }

    }
}

fn log_no_env_level<S: Into<String>>(level: S, msg: String) {
    let level: &str = &level.into();
    match level {
        "info" => info_no_env(msg),
        "warn" => warn_no_env(msg),
        "error" => error_no_env(msg),
        _ => {
            error_no_env(msg);
            return;
        },
    };
}