use std::error;
use std::sync::Arc;

use vulkano::command_buffer::{AutoCommandBuffer, DynamicState};
use vulkano::descriptor::DescriptorSet;
use vulkano::device::Queue;

pub trait Drawable {
    fn draw(
        &self,
        queue: Arc<Queue>,
        dynamic_state: &DynamicState,
        view_set: Arc<dyn DescriptorSet + Send + Sync>,
        lighting_set: Arc<dyn DescriptorSet + Send + Sync>,
    ) -> Result<AutoCommandBuffer, Box<dyn error::Error + Send + Sync>>;
}
