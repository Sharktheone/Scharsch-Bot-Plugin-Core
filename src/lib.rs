use jni::objects::{JClass};

pub mod config;
pub mod events;
pub mod websocket;
pub mod jni_utils;
pub mod plugin;


static mut CLASS: Option<JClass> = None;

pub fn set_class(class: JClass<'static>) {
    unsafe {
        CLASS = Some(class);
    }


}