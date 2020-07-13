use std::error;
use std::fmt;
use std::sync::Arc;

use crate::context::RenderContext;
use crate::mesh::Mesh;

use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::DescriptorSet;
use vulkano::device::Device;
use vulkano::format::Format;
use vulkano::framebuffer::{
    Framebuffer, FramebufferAbstract, RenderPassAbstract, RenderPassCreationError,
};
use vulkano::image::{AttachmentImage, ImageUsage, SwapchainImage};
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain;
use vulkano::swapchain::{
    AcquireError, ColorSpace, FullscreenExclusive, PresentMode, Surface, SurfaceTransform,
    Swapchain, SwapchainCreationError,
};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};

use winit::window::Window;

fn window_size_dependent_setup(
    device: Arc<Device>,
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };

    dynamic_state.viewports = Some(vec![viewport]);

    let depth_buffer = AttachmentImage::transient(device, dimensions, Format::D32Sfloat).unwrap();

    images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone())
                    .unwrap()
                    .add(depth_buffer.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}

#[derive(Debug)]
pub enum RendererCreationError {
    SwapchainError(SwapchainCreationError),
    RenderpassError(RenderPassCreationError),
}

impl fmt::Display for RendererCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RendererCreationError::SwapchainError(ref e) => e.fmt(f),
            RendererCreationError::RenderpassError(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for RendererCreationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            RendererCreationError::SwapchainError(ref e) => Some(e),
            RendererCreationError::RenderpassError(ref e) => Some(e),
        }
    }
}

impl From<SwapchainCreationError> for RendererCreationError {
    fn from(err: SwapchainCreationError) -> RendererCreationError {
        RendererCreationError::SwapchainError(err)
    }
}

impl From<RenderPassCreationError> for RendererCreationError {
    fn from(err: RenderPassCreationError) -> RendererCreationError {
        RendererCreationError::RenderpassError(err)
    }
}

pub struct Renderer {
    context: Arc<RenderContext>,
    surface: Arc<Surface<Window>>,
    swapchain: Arc<Swapchain<Window>>,
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
    dynamic_state: DynamicState,
    should_recreate_swapchain: bool,
}

impl Renderer {
    pub fn new(
        context: Arc<RenderContext>,
        surface: Arc<Surface<Window>>,
    ) -> Result<Self, RendererCreationError> {
        let caps = surface
            .capabilities(context.physical_device())
            .expect("failed to get surface capabilities");

        let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;

        let (swapchain, images) = Swapchain::new(
            context.device(),
            surface.clone(),
            caps.min_image_count,
            format,
            dimensions,
            1,
            ImageUsage::color_attachment(),
            &context.queue(),
            SurfaceTransform::Identity,
            alpha,
            PresentMode::Fifo,
            FullscreenExclusive::Default,
            true,
            ColorSpace::SrgbNonLinear,
        )?;

        let render_pass = Arc::new(vulkano::single_pass_renderpass!(context.device(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.format(),
                    samples: 1,
                },
                depth: {
                    load: Clear,
                    store: DontCare,
                    format: Format::D32Sfloat,
                    samples: 1,
                }
            },
        pass: {
            color: [color],
            depth_stencil: {depth}
        })?);

        let mut dynamic_state = DynamicState::none();

        let framebuffers = window_size_dependent_setup(
            context.device(),
            &images,
            render_pass.clone(),
            &mut dynamic_state,
        );

        Ok(Renderer {
            context,
            surface,
            swapchain,
            render_pass,
            framebuffers,
            dynamic_state,
            should_recreate_swapchain: false,
        })
    }

    pub fn render_pass(&self) -> Arc<dyn RenderPassAbstract + Send + Sync> {
        self.render_pass.clone()
    }

    pub fn resized(&mut self) {
        self.should_recreate_swapchain = true;
    }

    fn recreate_swapchain_if_needed(&mut self) {
        if self.should_recreate_swapchain {
            let dimensions: [u32; 2] = self.surface.window().inner_size().into();
            let (new_swapchain, new_images) = match self
                .swapchain
                .recreate_with_dimensions(dimensions)
            {
                Ok(r) => r,
                Err(vulkano::swapchain::SwapchainCreationError::UnsupportedDimensions) => return,
                Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
            };

            self.swapchain = new_swapchain;
            self.framebuffers = window_size_dependent_setup(
                self.context.device(),
                &new_images,
                self.render_pass.clone(),
                &mut self.dynamic_state,
            );
            self.should_recreate_swapchain = false;
        }
    }

    pub fn render<M: Mesh>(
        &mut self,
        mesh: &M,
        camera_descriptors: Arc<dyn DescriptorSet + Send + Sync>,
        lighting_descriptors: Arc<dyn DescriptorSet + Send + Sync>,
        mut previous_frame_end: Option<Box<dyn GpuFuture>>,
    ) -> Option<Box<dyn GpuFuture>> {
        self.recreate_swapchain_if_needed();

        let queue = self.context.queue();

        let (image_num, suboptimal, acquire_future) =
            match swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    self.should_recreate_swapchain = true;
                    return previous_frame_end;
                }
                Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };

        if suboptimal {
            self.should_recreate_swapchain = true;
        }

        let clear_values = vec![[0.1, 0.1, 0.1, 1.0].into(), 1f32.into()];

        let mut builder = AutoCommandBufferBuilder::primary_one_time_submit(
            self.context.device(),
            queue.family(),
        )
        .unwrap();

        builder
            .begin_render_pass(self.framebuffers[image_num].clone(), false, clear_values)
            .unwrap();

        let sub_command_buffer = mesh
            .draw(
                queue.clone(),
                &self.dynamic_state,
                camera_descriptors,
                lighting_descriptors,
            )
            .expect("Could not add mesh draw to cmd buffer");

        // executing a secondary command buffer is unsafe for now
        unsafe {
            builder.execute_commands(sub_command_buffer).unwrap();
        }

        builder.end_render_pass().unwrap();

        let command_buffer = builder.build().unwrap();

        let future = previous_frame_end
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(queue, self.swapchain.clone(), image_num)
            .then_signal_fence_and_flush();

        match future {
            Ok(future) => Some(future.boxed()),
            Err(FlushError::OutOfDate) => {
                self.should_recreate_swapchain = true;
                Some(sync::now(self.context.device()).boxed())
            }
            Err(e) => {
                println!("Failed to flush future: {:?}", e);
                Some(sync::now(self.context.device()).boxed())
            }
        }
    }
}
