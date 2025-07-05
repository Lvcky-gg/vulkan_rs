use ash::{Entry,Instance, vk};
use std::{error::Error, result::Result};
struct VulkanApp {
    _entry: Entry,
    instance: Instance,
}

impl VulkanApp {
    fn new() -> Result<Self,Box<dyn Error>> {
        log::debug!("Creating Application");
        let entry = unsafe {Entry::load()?};
        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0,1,0,0),
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
        };
        let instance: Instance = unsafe { entry.create_instance(&create_info, None)?};

        Ok(Self {
            _entry : entry,
            instance
        })
    }

    fn run(&mut self) {
        log::debug!("Running Application");
    }

    fn init_vulkan() {
         log::debug!("Init Vulkan");
    }
    fn main_loop() {
        log::debug!("Main Loop");       
    }
    fn cleanup() {
        log::debug!("cleanup");
    }
}

fn main() {
    env_logger::init();
    match VulkanApp::new() {
        Ok(mut app) => app.run(),
        Err(error) => log::error!("Failed to create Application. Cause: {}", error),
    }
}
