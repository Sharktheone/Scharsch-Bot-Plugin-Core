use jni::JNIEnv;
use jni::objects::{JClass};

pub mod config;
pub mod events;
pub mod websocket;
pub mod jni_utils;
pub mod plugin;


static mut ENV: Option<JNIEnv> = None;
static mut CLASS: Option<JClass> = None;

pub fn set_env(env: JNIEnv<'static>, class: JClass<'static>) {
    unsafe {
        ENV = Some(env);
        CLASS = Some(class);
    }


}