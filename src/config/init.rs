use std::io::{Write, Read};
use std::fs::File;
use std::path::PathBuf;

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

    let config_path = appdata_path().join("config.json");
    if !config_path.exists() {
        let default_config = r#"{
            "name": "Default User",
            "callsign": "N0CALL",
            "maidenhead": "FN31pr",
            "latitude": 0.0,
            "longitude": 0.0,
            "altitude": 0.0,
            "logbooks": []
        }"#;

        let mut file = File::create(config_path)?;
        file.write_all(default_config.as_bytes())?;
    }

    Ok(())
}
