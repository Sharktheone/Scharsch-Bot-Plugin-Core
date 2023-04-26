use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{ErrorKind, Write};
use rust_embed::RustEmbed;
use crate::config::config_format::{Config, CONFIG_PATH};
use crate::plugin::logger::error_no_env;

#[derive(RustEmbed)]
#[folder = "embed/"]
struct StandardConfig;

pub fn load_config() -> Result<Config, String> {
    let path = Path::new(CONFIG_PATH);

    let mut config_file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                let mut configfile = match File::create(CONFIG_PATH) {
                    Ok(file) => file,
                    Err(e) => {
                        return Err(format!("Error creating config file: {}", e));
                    }
                };
                let standard_config = match StandardConfig::get("config.json") {
                    Some(config) => config,
                    None => {
                        return Err("Error getting standard config".to_string());
                    }
                };

                match configfile.write(standard_config.data.as_ref()){
                    Ok(_) => {},
                    Err(e) => {
                        return Err(format!("Error writing standard config to file: {}", e));
                    }
                };
                return Err("Config file not found, created new one".to_string());
            } else {
                return Err(format!("Error opening config file: {}", e));
            }
        },
    };


    let mut config_string = String::new();
    match config_file.read_to_string(&mut config_string) {
        Ok(_) => {},
        Err(e) => {
            error_no_env(format!("Error reading config file: {}", e));
            return Err(format!("Error reading config file: {}", e));
        }
    };

    let config: Config = match serde_json::from_str(&config_string){
        Ok(config) => config,
        Err(e) => {
            error_no_env(format!("Error parsing config file: {}", e));
            return Err(format!("Error parsing config file: {}", e));
        }
    };
    return Ok(config);
}