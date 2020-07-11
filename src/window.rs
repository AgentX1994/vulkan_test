use std::sync::Arc;

use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::swapchain::{Capabilities, CapabilitiesError, Surface};
use vulkano_win::VkSurfaceBuild;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

pub struct RenderWindow {
    event_loop: EventLoop<()>,
    surface: Arc<Surface<Window>>,
}

impl RenderWindow {
    pub fn new(instance: Arc<Instance>) -> Self {
        let event_loop = EventLoop::new();
        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance)
            .unwrap();

        RenderWindow {
            event_loop,
            surface,
        }
    }

    pub fn run_event_loop<F>(self, event_handler: F)
    where
        F: 'static + FnMut(Event<'_, ()>, &EventLoopWindowTarget<()>, &mut ControlFlow),
    {
        self.event_loop.run(event_handler);
    }

    pub fn surface(&self) -> Arc<Surface<Window>> {
        self.surface.clone()
    }

    pub fn capabilities(
        &self,
        physical_device: PhysicalDevice,
    ) -> Result<Capabilities, CapabilitiesError> {
        self.surface.capabilities(physical_device)
    }
}
