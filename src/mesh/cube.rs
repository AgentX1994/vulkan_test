use crate::*;
use std::sync::Arc;

use vulkano::buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer, TypedBufferAccess};
use vulkano::device::Device;

use super::Mesh;

#[derive(Clone)]
pub struct Cube {
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
    pub fn new(device: Arc<Device>) -> Arc<Self> {
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
        Arc::new(Cube {
            vertex_buffer,
            index_buffer,
        })
    }
}

impl Mesh for Cube {
    fn is_indexed(&self) -> bool {
        true
    }

    fn vertex_buffer(&self) -> Arc<dyn BufferAccess + Send + Sync> {
        self.vertex_buffer.clone()
    }
    fn index_buffer(&self) -> Arc<dyn TypedBufferAccess<Content = [u32]> + Send + Sync> {
        self.index_buffer.clone()
    }
}
