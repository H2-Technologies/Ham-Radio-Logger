use std::io::{Write, Read};
use std::fs::File;
use std::path::PathBuf;

use crate::structs::Configuration;

fn appdata_path() -> PathBuf {
    let mut path = std::env::var("APPDATA")
        .unwrap_or_else(|_| String::from("."));
    path.push_str("\\HamLog");
    let appdata_path = PathBuf::from(path);
    appdata_path
}

fn create_appdata_dir() -> std::io::Result<()> {
    let appdata_path = appdata_path();
    if !appdata_path.exists() {
        std::fs::create_dir_all(&appdata_path)?;
    }
    Ok(())
}

pub fn init_config() -> std::io::Result<()> {
    create_appdata_dir()?;
    let default_config = Configuration {
        name: String::from("Default User"),
        callsign: String::from("N0CALL"),
        maidenhead: String::from("FN31pr"),
        latitude: 0u64,
        longitude: 0u64,
        altitude: 0u64,
        logbooks: Vec::new(),
    };
    let config_path = appdata_path().join("config.json");
    if !config_path.exists() {
        let default_config = serde_json::to_string_pretty(&default_config)
            .expect("Failed to serialize default configuration");
        let mut file = File::create(&config_path)?;
        file.write_all(default_config.as_bytes())?;
        println!("Default configuration created at {:?}", config_path);
    }

    Ok(())
}
