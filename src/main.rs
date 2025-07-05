use ash::{Entry, Instance, vk};
use std::{error::Error, result::Result};
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::Window, window::WindowBuilder};
struct VulkanApp {
    _event_loop: EventLoop<()>,
    // _window: Window,
    _entry: Entry,
    instance: Instance,
}

impl VulkanApp {
    fn new() -> Result<Self, Box<dyn Error>> {
        log::debug!("Creating Application");

        let event_loop = EventLoop::new()?;
        let entry: Entry = unsafe { Entry::load()? };
        // let window = WindowBuilder::new()
        //     .with_title("Vulkan tutorial with Ash")
        //     .with_
        let app_info: vk::ApplicationInfo<'_> = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };
        let create_info: vk::InstanceCreateInfo<'_> = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
        };
        let instance: Instance = unsafe { entry.create_instance(&create_info, None)? };

        Ok(Self {
            _event_loop: event_loop,
            // _window: window,
            _entry: entry,
            instance,
        })
    }

    fn run(&mut self) {
        log::debug!("Running Application");
    }
}

fn main() {
    env_logger::init();
    match VulkanApp::new() {
        Ok(mut app) => app.run(),
        Err(error) => log::error!("Failed to create Application. Cause: {}", error),
    }
}
