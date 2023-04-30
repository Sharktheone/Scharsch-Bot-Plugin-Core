use jni::JavaVM;
use crate::plugin::logger::error_no_env;

pub mod config;
pub mod events;
pub mod websocket;
pub mod jni_utils;
pub mod plugin;

static mut VM: Option<JavaVM> = None;

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