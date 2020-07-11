pub mod cube;

use std::error;
use std::sync::Arc;

use vulkano::command_buffer::{AutoCommandBuffer, DynamicState};
use vulkano::descriptor::DescriptorSet;
use vulkano::device::Queue;

use crate::material::Material;

pub trait Mesh {
    fn set_material(&mut self, material: Arc<dyn Material + Send + Sync>);
    fn draw(
        &self,
        queue: Arc<Queue>,
        dynamic_state: &DynamicState,
        view_set: Arc<dyn DescriptorSet + Send + Sync>,
        lighting_set: Arc<dyn DescriptorSet + Send + Sync>,
    ) -> Result<AutoCommandBuffer, Box<dyn error::Error + Send + Sync>>;
}
