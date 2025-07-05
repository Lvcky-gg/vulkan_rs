use std::{error::Error, result::Result};

struct VulkanApp;

impl VulkanApp {
    fn new() -> Result<Self, Box<dyn Error>> {
        log::info!("Creating Application");
        Ok(Self)
    }
    fn run(&mut self) {
        log::info!("Running Application");
    }
}

fn main() {
    env_logger::init();
    match VulkanApp::new() {
        Ok(mut app) => app.run(),
        Err(error) => log::error!("Failed to create Application. Cause: {}", error),
    }
}
