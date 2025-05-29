mod structs;
mod config;

fn main() {
    // Initialize application data directory
    if let Err(e) = config::init::init_config() {
        eprintln!("Failed to initialize config: {}", e);
        return;
    }
    
    println!("Hello, world!");
}
