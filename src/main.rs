mod structs;
mod config;

fn main() {
    // Initialize application data directory
    match config::init::read_config() {
        Ok(_) => println!("Configuration initialized successfully."),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            if let Err(e) = config::init::init_config() {
                eprintln!("Failed to create default configuration: {}", e);
            } else {
                println!("Default configuration created successfully.");
            }
        }
        Err(e) => eprintln!("Failed to initialize configuration: {}", e),
    }
    println!("Hello, world!");
}
