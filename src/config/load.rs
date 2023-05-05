use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{ErrorKind, Write};
use rust_embed::RustEmbed;
use crate::config::config_format::{Config, CONFIG_PATH};
use crate::plugin::logger::{error};

#[derive(RustEmbed)]
#[folder = "embed/"]
struct StandardConfig;

pub static mut CONFIG: Option<Config> = None;
pub static mut CONFIG_LOADED: bool = false;

pub fn load_config() -> Result<Config, String> {
    let path = Path::new(CONFIG_PATH);

    let mut config_file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            return if e.kind() == ErrorKind::NotFound {
                let path = Path::new(CONFIG_PATH);
                let dir = match path.parent() {
                    Some(dir) => dir,
                    None => {
                        print_disabled();
                        return Err("Error getting config directory".to_string());
                    }
                };
                let mut configfile = match std::fs::create_dir_all(dir) {
                    Ok(_) => match File::create(path) {
                        Ok(file) => file,
                        Err(e) => {
                            print_disabled();
                            return Err(format!("Error creating config file: {}", e));
                        },
                    },
                    Err(e) => {
                        print_disabled();
                        return Err(format!("Error creating plugin directory: {}", e));
                    }
                };
                let standard_config = match StandardConfig::get("config.yml") {
                    Some(config) => config,
                    None => {
                        print_disabled();
                        return Err("Error getting standard config".to_string());
                    }
                };

                match configfile.write(standard_config.data.as_ref()) {
                    Ok(_) => {},
                    Err(e) => {
                        print_disabled();
                        return Err(format!("Error writing standard config to file: {}", e));
                    }
                };
                let msg = r#"
    ╭─────────────────────────────────────────────────────────────────╮
    │                                                                 │
    │             Config file not found, created new one!             │
    │       Please edit the config file and restart the server!       │
    │                                                                 │
    ╰─────────────────────────────────────────────────────────────────╯"#;

                error(msg);

                Err("Config file not found, created new one".to_string())
            } else {
                print_disabled();
                Err(format!("Error opening config file: {}", e))
            }
        },
    };


    let mut config_string = String::new();
    match config_file.read_to_string(&mut config_string) {
        Ok(_) => {},
        Err(e) => {
            print_disabled();
            return Err(format!("Error reading config file: {}", e));
        }
    };

    let config: Config = match serde_yaml::from_str(&config_string){
        Ok(config) => config,
        Err(e) => {
            error(format!("Error parsing config file: {}", e));
            return {
                print_disabled();
                Err(format!("Error parsing config file: {}", e))
            };
        }
    };
    unsafe {
        CONFIG = Some(config.clone());
        CONFIG_LOADED = true;
    }
    return Ok(config);
}

fn print_disabled() {

    let msg = r#"
    ╭─────────────────────────────────────────────────────────────────╮
    │                                                                 │
    │                   Failed to load config file!                   │
    │       Please edit the config file and restart the server!       │
    │                                                                 │
    ╰─────────────────────────────────────────────────────────────────╯"#;

    error(msg);
}