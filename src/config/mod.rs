use std::{env, fs, path::{Path, PathBuf}};
use std::io::Write;
use toml;
use lazy_static::lazy_static;
use crate::config::structs::{BackuperConfig};

pub mod structs;

lazy_static! {
    static ref CONFIG_PATH: PathBuf = env::current_exe().unwrap().parent().unwrap().join("config.toml");
}

pub fn init() -> Result<BackuperConfig, String> {
    let config_path = get_config_path();

    println!("Config path is: {}", config_path.display());
    match fs::exists(config_path) {
        Ok(false) | Err(_) => {
            eprint!("Config not found, creating new one.");
            make_config()
        }
        Ok(true) => {
            println!("Config found, reading.");
            read_config()
        }
    }
}

fn make_config() -> Result<BackuperConfig, String> {
    let new_config = structs::BackuperConfig::new();

    match toml::to_string(&new_config) {
        Ok(toml) => {
            let f = fs::File::create_new(get_config_path());
            match f {
                Ok(_) => {
                    if let Ok(_) = f.unwrap().write_all(toml.as_bytes()) {
                        return Ok(new_config)
                    }

                    Err("Could not write to config file.".to_string())
                },
                Err(err) => {
                    Err(format!("Error creating config file: {}", err))
                }
            }
        },
        Err(err) => {
            return Err(format!("Error reading config: {}", err))
        }
    }
}

fn read_config() -> Result<BackuperConfig, String> {
    if let Ok(config_contents) = fs::read_to_string(get_config_path()) {
        return match toml::from_str(&config_contents) {
            Ok(config) => Ok(config),
            Err(err) => {
                Err(format!("Error reading config: {}", err))
            }
        }
    }

    Err("Failed to read config".to_string())
}

pub fn get_config_path() -> &'static Path {
    &*CONFIG_PATH
}