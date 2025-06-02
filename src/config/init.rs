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
        maidenhead: String::from("EN80TU"),
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

pub fn read_config() -> std::io::Result<Configuration> {
    let config_path = appdata_path().join("config.json");
    if !config_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Configuration file not found",
        ));
    }

    let mut file = File::open(&config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Configuration = serde_json::from_str(&contents)
        .expect("Failed to deserialize configuration");

    println!("Configuration loaded from {:?}", config_path);
    println!("Name: {}", config.name);
    println!("Callsign: {}", config.callsign);
    println!("Maidenhead: {}", config.maidenhead);
    println!("Latitude: {}", config.latitude);
    println!("Longitude: {}", config.longitude);
    println!("Altitude: {}", config.altitude);
    println!("Logbooks: {}", config.logbooks.len());
    
    Ok(config)
}