use std::error;
use std::sync::Arc;

use vulkano::buffer::CpuBufferPool;
use vulkano::command_buffer::{AutoCommandBuffer, AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor::DescriptorSet;
use vulkano::device::{Device, Queue};
use vulkano::pipeline::GraphicsPipelineAbstract;

use nalgebra_glm as glm;

use crate::drawable::Drawable;
use crate::material::Material;
use crate::mesh::Mesh;

// TODO HOW THE HELL DO WE DEAL WITH UNIFORM TYPES
use crate::material::phong::vs::ty::world_matrix;

pub struct SceneObject {
    transform: glm::Mat4,
    material: Arc<dyn Material + Send + Sync>,
    mesh: Arc<dyn Mesh + Send + Sync>,
    uniform_buffer_pool: CpuBufferPool<world_matrix>,
}

impl SceneObject {
    pub fn new(
        device: Arc<Device>,
        material: Arc<dyn Material + Send + Sync>,
        mesh: Arc<dyn Mesh + Send + Sync>,
    ) -> Self {
        let uniform_buffer_pool: CpuBufferPool<world_matrix> =
            CpuBufferPool::uniform_buffer(device);
        SceneObject {
            uniform_buffer_pool,
            transform: glm::identity(),
            material,
            mesh,
        }
    }

    pub fn get_transform(&self) -> glm::Mat4 {
        self.transform
    }

    pub fn set_transform(&mut self, transform: glm::Mat4) {
        self.transform = transform;
    }

    pub fn get_material(&self) -> Arc<dyn Material + Send + Sync> {
        self.material.clone()
    }

    pub fn set_material(&mut self, material: Arc<dyn Material + Send + Sync>) {
        self.material = material;
    }

    pub fn get_mesh(&self) -> Arc<dyn Mesh + Send + Sync> {
        self.mesh.clone()
    }
    pub fn set_mesh(&mut self, mesh: Arc<dyn Mesh + Send + Sync>) {
        self.mesh = mesh;
    }
}

impl Drawable for SceneObject {
    fn draw(
        &self,
        queue: Arc<Queue>,
        dynamic_state: &DynamicState,
        view_set: Arc<dyn DescriptorSet + Send + Sync>,
        lighting_set: Arc<dyn DescriptorSet + Send + Sync>,
    ) -> Result<AutoCommandBuffer, Box<dyn error::Error + Send + Sync>> {
        let transform_uniform_data = world_matrix {
            world: self.transform.into(),
        };

        let uniforms = self.uniform_buffer_pool.next(transform_uniform_data)?;

        let layout = self.material.get_world_layout();
        let world_set = Arc::new(
            PersistentDescriptorSet::start(layout)
                .add_buffer(uniforms)
                .unwrap()
                .build()
                .unwrap(),
        );

        let pipeline = self.material.pipeline();
        let mut builder = AutoCommandBufferBuilder::secondary_graphics(
            pipeline.device().clone(),
            queue.family(),
            pipeline.clone().subpass(),
        )?;
        if self.mesh.is_indexed() {
            builder.draw_indexed(
                pipeline.clone(),
                &dynamic_state,
                vec![self.mesh.vertex_buffer()],
                self.mesh.index_buffer(),
                (
                    view_set,
                    world_set,
                    lighting_set,
                    self.material.material_descriptors(),
                ),
                (),
            )?;
        } else {
            builder.draw(
                pipeline,
                dynamic_state,
                vec![self.mesh.vertex_buffer()],
                (
                    view_set,
                    world_set,
                    lighting_set,
                    self.material.material_descriptors(),
                ),
                (),
            )?;
        }
        Ok(builder.build()?)
    }
}
