mod structs;
mod config;

fn main() {
    // Initialize application data directory
    match config::init::init_config() {
        Ok(_) => println!("Configuration initialized successfully."),
        Err(e) => eprintln!("Failed to initialize configuration: {}", e),
    }
    println!("Hello, world!");
}
