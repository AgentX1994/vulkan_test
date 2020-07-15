pub mod camera;
pub mod context;
pub mod controller;
pub mod drawable;
pub mod input;
pub mod material;
pub mod mesh;
pub mod renderer;
pub mod scene;
pub mod utility;
pub mod window;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

vulkano::impl_vertex!(Vertex, position, normal, uv);
