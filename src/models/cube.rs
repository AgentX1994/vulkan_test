use crate::*;
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, DrawError, DrawIndexedError, DynamicState,
};
use vulkano::descriptor::descriptor_set::DescriptorSet;
use vulkano::device::Device;
use vulkano::pipeline::GraphicsPipelineAbstract;

#[derive(Clone)]
pub struct Cube {
    position: [f32; 3],
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

// const VERTICES: [Vertex; 36] = [
//     // positions          // normals           // texture coords
//     Vertex {
//         position: [-0.5, -0.5, -0.5],
//         normal: [0.0, 0.0, -1.0],
//         uv: [0.0, 0.0],
//     },
//     Vertex {
//         position: [0.5, -0.5, -0.5],
//         normal: [0.0, 0.0, -1.0],
//         uv: [1.0, 0.0],
//     },
//     Vertex {
//         position: [0.5, 0.5, -0.5],
//         normal: [0.0, 0.0, -1.0],
//         uv: [1.0, 1.0],
//     },
//     Vertex {
//         position: [0.5, 0.5, -0.5],
//         normal: [0.0, 0.0, -1.0],
//         uv: [1.0, 1.0],
//     },
//     Vertex {
//         position: [-0.5, 0.5, -0.5],
//         normal: [0.0, 0.0, -1.0],
//         uv: [0.0, 1.0],
//     },
//     Vertex {
//         position: [-0.5, -0.5, -0.5],
//         normal: [0.0, 0.0, -1.0],
//         uv: [0.0, 0.0],
//     },
//     Vertex {
//         position: [-0.5, -0.5, 0.5],
//         normal: [0.0, 0.0, 1.0],
//         uv: [0.0, 0.0],
//     },
//     Vertex {
//         position: [0.5, -0.5, 0.5],
//         normal: [0.0, 0.0, 1.0],
//         uv: [1.0, 0.0],
//     },
//     Vertex {
//         position: [0.5, 0.5, 0.5],
//         normal: [0.0, 0.0, 1.0],
//         uv: [1.0, 1.0],
//     },
//     Vertex {
//         position: [0.5, 0.5, 0.5],
//         normal: [0.0, 0.0, 1.0],
//         uv: [1.0, 1.0],
//     },
//     Vertex {
//         position: [-0.5, 0.5, 0.5],
//         normal: [0.0, 0.0, 1.0],
//         uv: [0.0, 1.0],
//     },
//     Vertex {
//         position: [-0.5, -0.5, 0.5],
//         normal: [0.0, 0.0, 1.0],
//         uv: [0.0, 0.0],
//     },
//     Vertex {
//         position: [-0.5, 0.5, 0.5],
//         normal: [-1.0, 0.0, 0.0],
//         uv: [1.0, 0.0],
//     },
//     Vertex {
//         position: [-0.5, 0.5, -0.5],
//         normal: [-1.0, 0.0, 0.0],
//         uv: [1.0, 1.0],
//     },
//     Vertex {
//         position: [-0.5, -0.5, -0.5],
//         normal: [-1.0, 0.0, 0.0],
//         uv: [0.0, 1.0],
//     },
//     Vertex {
//         position: [-0.5, -0.5, -0.5],
//         normal: [-1.0, 0.0, 0.0],
//         uv: [0.0, 1.0],
//     },
//     Vertex {
//         position: [-0.5, -0.5, 0.5],
//         normal: [-1.0, 0.0, 0.0],
//         uv: [0.0, 0.0],
//     },
//     Vertex {
//         position: [-0.5, 0.5, 0.5],
//         normal: [-1.0, 0.0, 0.0],
//         uv: [1.0, 0.0],
//     },
//     Vertex {
//         position: [0.5, 0.5, 0.5],
//         normal: [1.0, 0.0, 0.0],
//         uv: [1.0, 0.0],
//     },
//     Vertex {
//         position: [0.5, 0.5, -0.5],
//         normal: [1.0, 0.0, 0.0],
//         uv: [1.0, 1.0],
//     },
//     Vertex {
//         position: [0.5, -0.5, -0.5],
//         normal: [1.0, 0.0, 0.0],
//         uv: [0.0, 1.0],
//     },
//     Vertex {
//         position: [0.5, -0.5, -0.5],
//         normal: [1.0, 0.0, 0.0],
//         uv: [0.0, 1.0],
//     },
//     Vertex {
//         position: [0.5, -0.5, 0.5],
//         normal: [1.0, 0.0, 0.0],
//         uv: [0.0, 0.0],
//     },
//     Vertex {
//         position: [0.5, 0.5, 0.5],
//         normal: [1.0, 0.0, 0.0],
//         uv: [1.0, 0.0],
//     },
//     Vertex {
//         position: [-0.5, -0.5, -0.5],
//         normal: [0.0, -1.0, 0.0],
//         uv: [0.0, 1.0],
//     },
//     Vertex {
//         position: [0.5, -0.5, -0.5],
//         normal: [0.0, -1.0, 0.0],
//         uv: [1.0, 1.0],
//     },
//     Vertex {
//         position: [0.5, -0.5, 0.5],
//         normal: [0.0, -1.0, 0.0],
//         uv: [1.0, 0.0],
//     },
//     Vertex {
//         position: [0.5, -0.5, 0.5],
//         normal: [0.0, -1.0, 0.0],
//         uv: [1.0, 0.0],
//     },
//     Vertex {
//         position: [-0.5, -0.5, 0.5],
//         normal: [0.0, -1.0, 0.0],
//         uv: [0.0, 0.0],
//     },
//     Vertex {
//         position: [-0.5, -0.5, -0.5],
//         normal: [0.0, -1.0, 0.0],
//         uv: [0.0, 1.0],
//     },
//     Vertex {
//         position: [-0.5, 0.5, -0.5],
//         normal: [0.0, 1.0, 0.0],
//         uv: [0.0, 1.0],
//     },
//     Vertex {
//         position: [0.5, 0.5, -0.5],
//         normal: [0.0, 1.0, 0.0],
//         uv: [1.0, 1.0],
//     },
//     Vertex {
//         position: [0.5, 0.5, 0.5],
//         normal: [0.0, 1.0, 0.0],
//         uv: [1.0, 0.0],
//     },
//     Vertex {
//         position: [0.5, 0.5, 0.5],
//         normal: [0.0, 1.0, 0.0],
//         uv: [1.0, 0.0],
//     },
//     Vertex {
//         position: [-0.5, 0.5, 0.5],
//         normal: [0.0, 1.0, 0.0],
//         uv: [0.0, 0.0],
//     },
//     Vertex {
//         position: [-0.5, 0.5, -0.5],
//         normal: [0.0, 1.0, 0.0],
//         uv: [0.0, 1.0],
//     },
// ];

impl Cube {
    pub fn new(position: [f32; 3], device: Arc<Device>) -> Self {
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            false,
            VERTICES.iter().map(|&x| x),
        )
        .unwrap();

        let index_buffer = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage::all(),
            false,
            INDICES.iter().map(|&x| x),
        )
        .unwrap();
        Cube {
            position,
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn draw<'a>(
        &self,
        cmd_buffer: &'a mut AutoCommandBufferBuilder,
        pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
        view_set: Arc<dyn DescriptorSet + Send + Sync>,
        lighting_set: Arc<dyn DescriptorSet + Send + Sync>,
    ) -> Result<&'a mut AutoCommandBufferBuilder, DrawIndexedError> {
        cmd_buffer.draw_indexed(
            pipeline.clone(),
            &DynamicState::none(),
            vec![self.vertex_buffer.clone()],
            self.index_buffer.clone(),
            (view_set, lighting_set),
            (),
        )
    }
}
