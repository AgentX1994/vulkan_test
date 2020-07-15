use std::sync::Arc;
use vulkano::descriptor::descriptor_set::UnsafeDescriptorSetLayout;
use vulkano::descriptor::DescriptorSet;
use vulkano::pipeline::GraphicsPipelineAbstract;

pub trait Material {
    fn get_world_layout(&self) -> Arc<UnsafeDescriptorSetLayout>;
    fn get_view_layout(&self) -> Arc<UnsafeDescriptorSetLayout>;
    fn get_lighting_layout(&self) -> Arc<UnsafeDescriptorSetLayout>;
    fn pipeline(&self) -> Arc<dyn GraphicsPipelineAbstract + Send + Sync>;
    fn material_descriptors(&self) -> Arc<dyn DescriptorSet + Send + Sync>;
}

pub mod phong;
