use crate::*;
use std::error;
use std::sync::Arc;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBuffer, AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::DescriptorSet;
use vulkano::device::{Device, Queue};
use vulkano::pipeline::GraphicsPipelineAbstract;

use super::Mesh;
use crate::material::Material;

#[derive(Clone)]
pub struct Cube {
    material: Arc<dyn Material>,
    vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
}

const VERTICES: [Vertex; 24] = [
    // first face - front (0 - 3)
    Vertex {
        position: [-1.0, -1.0, -1.0],
        normal: [0.0, 0.0, -1.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, -1.0],
        normal: [0.0, 0.0, -1.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0],
        normal: [0.0, 0.0, -1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
        normal: [0.0, 0.0, -1.0],
        uv: [1.0, 1.0],
    },
    // second face - bottom (4 - 7)
    Vertex {
        position: [1.0, -1.0, -1.0],
        normal: [0.0, -1.0, 0.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0],
        normal: [0.0, -1.0, 0.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
        normal: [0.0, -1.0, 0.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0],
        normal: [0.0, -1.0, 0.0],
        uv: [1.0, 1.0],
    },
    // third face - right (8 - 11)
    Vertex {
        position: [1.0, -1.0, -1.0],
        normal: [1.0, 0.0, 0.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
        normal: [1.0, 0.0, 0.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
        normal: [1.0, 0.0, 0.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        normal: [1.0, 0.0, 0.0],
        uv: [1.0, 1.0],
    },
    // fourth face - left (12 - 15)
    Vertex {
        position: [-1.0, -1.0, 1.0],
        normal: [-1.0, 0.0, 0.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, -1.0],
        normal: [-1.0, 0.0, 0.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
        normal: [-1.0, 0.0, 0.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, -1.0],
        normal: [-1.0, 0.0, 0.0],
        uv: [1.0, 1.0],
    },
    // fifth face - back (16 - 19)
    Vertex {
        position: [1.0, -1.0, 1.0],
        normal: [0.0, 0.0, 1.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0],
        normal: [0.0, 0.0, 1.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        normal: [0.0, 0.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
        normal: [0.0, 0.0, 1.0],
        uv: [1.0, 1.0],
    },
    // sixth face - top (20 - 23)
    Vertex {
        position: [-1.0, 1.0, -1.0],
        normal: [0.0, 1.0, 0.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, -1.0],
        normal: [0.0, 1.0, 0.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 1.0],
        normal: [0.0, 1.0, 0.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0],
        normal: [0.0, 1.0, 0.0],
        uv: [1.0, 1.0],
    },
];

const INDICES: [u32; 36] = [
    0, 1, 2, 2, 1, 3, 4, 5, 6, 6, 5, 7, 8, 9, 10, 10, 9, 11, 12, 13, 14, 14, 13, 15, 16, 17, 18,
    18, 17, 19, 20, 21, 22, 22, 21, 23,
];

impl Cube {
    pub fn new(device: Arc<Device>, material: Arc<dyn Material>) -> Self {
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            false,
            VERTICES.iter().copied(),
        )
        .unwrap();

        let index_buffer = CpuAccessibleBuffer::from_iter(
            device,
            BufferUsage::all(),
            false,
            INDICES.iter().copied(),
        )
        .unwrap();
        Cube {
            material,
            vertex_buffer,
            index_buffer,
        }
    }
}

impl Mesh for Cube {
    fn set_material(&mut self, material: Arc<dyn Material + Send + Sync>) {
        self.material = material;
    }

    fn draw(
        &self,
        queue: Arc<Queue>,
        dynamic_state: &DynamicState,
        view_set: Arc<dyn DescriptorSet + Send + Sync>,
        lighting_set: Arc<dyn DescriptorSet + Send + Sync>,
    ) -> Result<AutoCommandBuffer, Box<dyn error::Error + Send + Sync>> {
        let pipeline = self.material.pipeline();
        let mut builder = AutoCommandBufferBuilder::secondary_graphics(
            pipeline.device().clone(),
            queue.family(),
            pipeline.clone().subpass(),
        )?;
        builder.draw_indexed(
            pipeline.clone(),
            &dynamic_state,
            vec![self.vertex_buffer.clone()],
            self.index_buffer.clone(),
            (view_set, lighting_set, self.material.material_descriptors()),
            (),
        )?;
        Ok(builder.build()?)
    }
}
