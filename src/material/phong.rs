use std::error;
use std::sync::Arc;

use vulkano::buffer::{BufferUsage, ImmutableBuffer};
use vulkano::command_buffer::AutoCommandBuffer;
use vulkano::command_buffer::CommandBufferExecFuture;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor::descriptor_set::UnsafeDescriptorSetLayout;
use vulkano::descriptor::DescriptorSet;
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::{RenderPassAbstract, Subpass};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::sync::NowFuture;

use super::Material;
use crate::Vertex;
use nalgebra_glm as glm;

pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "shaders/normal.vert"
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "shaders/shading.frag"
    }
}

pub struct Phong {
    buffer: Arc<ImmutableBuffer<fs::ty::material_parameters>>,
    pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
    material_descriptors: Arc<dyn DescriptorSet + Send + Sync>,
}

impl Phong {
    pub fn new(
        ambient: glm::Vec3,
        diffuse: glm::Vec3,
        specular: glm::Vec3,
        device: Arc<Device>,
        queue: Arc<Queue>,
        render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    ) -> Result<
        (
            Arc<Self>,
            CommandBufferExecFuture<NowFuture, AutoCommandBuffer>,
        ),
        Box<dyn error::Error + Send + Sync>,
    > {
        let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
        let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");

        let pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<Vertex>()
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .depth_stencil_simple_depth()
                .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                .build(device.clone())?,
        );

        let material_uniform_data = fs::ty::material_parameters {
            material: fs::ty::Material {
                ambient: ambient.into(),
                diffuse: diffuse.into(),
                specular: specular.into(),
                shininess: 20.0,
                _dummy0: [0, 0, 0, 0],
                _dummy1: [0, 0, 0, 0],
            },
        };

        let (buffer, future) =
            ImmutableBuffer::from_data(material_uniform_data, BufferUsage::all(), queue)?;

        let layout = pipeline.descriptor_set_layout(2).unwrap();
        let material_descriptors = Arc::new(
            PersistentDescriptorSet::start(layout.clone())
                .add_buffer(buffer.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        let phong = Arc::new(Phong {
            buffer,
            pipeline,
            material_descriptors,
        });

        Ok((phong, future))
    }
}

impl Material for Phong {
    fn pipeline(&self) -> Arc<dyn GraphicsPipelineAbstract + Send + Sync> {
        return self.pipeline.clone();
    }
    fn material_descriptors(&self) -> Arc<dyn DescriptorSet + Send + Sync> {
        return self.material_descriptors.clone();
    }

    fn get_view_layout(&self) -> Arc<UnsafeDescriptorSetLayout> {
        self.pipeline.descriptor_set_layout(0).unwrap().clone()
    }
    fn get_lighting_layout(&self) -> Arc<UnsafeDescriptorSetLayout> {
        self.pipeline.descriptor_set_layout(1).unwrap().clone()
    }
}