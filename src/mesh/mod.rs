pub mod cube;

use std::sync::Arc;

use vulkano::buffer::{BufferAccess, TypedBufferAccess};

pub trait Mesh {
    fn is_indexed(&self) -> bool;

    fn vertex_buffer(&self) -> Arc<dyn BufferAccess + Send + Sync>;
    fn index_buffer(&self) -> Arc<dyn TypedBufferAccess<Content = [u32]> + Send + Sync>;
}
