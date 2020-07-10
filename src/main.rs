use std::iter;
use std::sync::Arc;
use std::time::Instant;

extern crate nalgebra_glm as glm;

use vulkan_test::models::cube::Cube;
use vulkan_test::Vertex;

use vulkano::buffer::cpu_pool::CpuBufferPool;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::device::{Device, DeviceExtensions, Features};
use vulkano::format::Format;
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract, Subpass};
use vulkano::image::attachment::AttachmentImage;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::swapchain;
use vulkano::swapchain::{
    AcquireError, ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain,
};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};
use vulkano_win::VkSurfaceBuild;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "shaders/normal.vert"
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "shaders/shading.frag"
    }
}

fn window_size_dependent_setup(
    device: Arc<Device>,
    vs: &vs::Shader,
    fs: &fs::Shader,
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    aspect_ratio: &mut f32,
) -> (
    Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
    Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
) {
    let dimensions = images[0].dimensions();
    *aspect_ratio = dimensions[0] as f32 / dimensions[1] as f32;

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };

    let depth_buffer =
        AttachmentImage::transient(device.clone(), dimensions, Format::D32Sfloat).unwrap();

    let framebuffers = images
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
        .collect::<Vec<_>>();
    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .viewports(iter::once(viewport))
            .fragment_shader(fs.main_entry_point(), ())
            .depth_stencil_simple_depth()
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );

    (pipeline, framebuffers)
}

fn main() {
    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("failed to create instance")
    };

    let events_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&events_loop, instance.clone())
        .unwrap();

    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    for family in physical.queue_families() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queues_count()
        );
    }

    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    let (device, mut queues) = {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions {
                khr_storage_buffer_storage_class: true,
                khr_swapchain: true,
                ..DeviceExtensions::none()
            },
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };

    let queue = queues.next().unwrap();

    let caps = surface
        .capabilities(physical)
        .expect("failed to get surface capabilities");

    let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;

    let (mut swapchain, images) = Swapchain::new(
        device.clone(),
        surface.clone(),
        caps.min_image_count,
        format,
        dimensions,
        1,
        ImageUsage::color_attachment(),
        &queue,
        SurfaceTransform::Identity,
        alpha,
        PresentMode::Fifo,
        FullscreenExclusive::Default,
        true,
        ColorSpace::SrgbNonLinear,
    )
    .expect("failed to create swapchain");

    let vertex_shader = vs::Shader::load(device.clone()).expect("failed to create shader module");
    let frag_shader = fs::Shader::load(device.clone()).expect("failed to create shader module");

    let render_pass = Arc::new(
        vulkano::single_pass_renderpass!(device.clone(),
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
        })
        .unwrap(),
    );

    let cube = Cube::new([0.0, 0.0, 0.0], device.clone());

    // let mut dynamic_state = DynamicState::none();
    let mut aspect_ratio = 1280.0 / 1024.0;

    let (mut pipeline, mut framebuffers) = window_size_dependent_setup(
        device.clone(),
        &vertex_shader,
        &frag_shader,
        &images,
        render_pass.clone(),
        &mut aspect_ratio,
    );

    let mut recreate_swapchain = false;
    let mut previous_frame_end = Some(sync::now(device.clone()).boxed());

    // for timing
    let start_time = Instant::now();
    let view_uniform_buffer_pool = CpuBufferPool::uniform_buffer(device.clone());
    let lighting_uniform_buffer_pool = CpuBufferPool::uniform_buffer(device.clone());

    events_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            recreate_swapchain = true;
        }
        Event::RedrawEventsCleared => {
            previous_frame_end.as_mut().unwrap().cleanup_finished();
            if recreate_swapchain {
                let dimensions: [u32; 2] = surface.window().inner_size().into();
                aspect_ratio = dimensions[1] as f32 / dimensions[0] as f32;
                let (new_swapchain, new_images) =
                    match swapchain.recreate_with_dimensions(dimensions) {
                        Ok(r) => r,
                        Err(vulkano::swapchain::SwapchainCreationError::UnsupportedDimensions) => {
                            return
                        }
                        Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                    };

                swapchain = new_swapchain;
                let results = window_size_dependent_setup(
                    device.clone(),
                    &vertex_shader,
                    &frag_shader,
                    &new_images,
                    render_pass.clone(),
                    &mut aspect_ratio,
                );
                pipeline = results.0;
                framebuffers = results.1;
                recreate_swapchain = false;
            }

            let (image_num, suboptimal, acquire_future) =
                match swapchain::acquire_next_image(swapchain.clone(), None) {
                    Ok(r) => r,
                    Err(AcquireError::OutOfDate) => {
                        recreate_swapchain = true;
                        return;
                    }
                    Err(e) => panic!("Failed to acquire next image: {:?}", e),
                };

            if suboptimal {
                recreate_swapchain = true;
            }

            let clear_values = vec![[0.1, 0.1, 0.1, 1.0].into(), 1f32.into()];

            let mut builder =
                AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                    .unwrap();

            // Calculate view
            let elapsed_time = start_time.elapsed();
            let elapsed_time_secs = elapsed_time.as_secs_f32();
            let camera_pos = glm::vec3(
                2.0 * elapsed_time_secs.cos(),
                2.0f32,
                2.0 * elapsed_time_secs.sin(),
            );

            let projection = glm::perspective(aspect_ratio, 45.0, 0.1, 100.0);

            // Vulkan requires us to reverse the y axis for some reason
            // Do this by setting up to -1
            let view = glm::look_at(
                &camera_pos,
                &glm::vec3(0.0, 0.0, 0.0),
                &glm::vec3(0.0, -1.0, 0.0),
            );

            let should_print = false;
            if should_print {
                println!("view: ");
                println!(
                    "[{:10},{:10},{:10},{:10},",
                    view[0], view[4], view[8], view[12]
                );
                println!(
                    "{:10},{:10},{:10},{:10},",
                    view[1], view[5], view[9], view[13]
                );
                println!(
                    "{:10},{:10},{:10},{:10},",
                    view[2], view[6], view[10], view[14]
                );
                println!(
                    "{:10},{:10},{:10},{:10}]",
                    view[3], view[7], view[11], view[15]
                );
                println!("");
                println!("projection: ");
                println!(
                    "[{:10},{:10},{:10},{:10},",
                    projection[0], projection[4], projection[8], projection[12]
                );
                println!(
                    "{:10},{:10},{:10},{:10},",
                    projection[1], projection[5], projection[9], projection[13]
                );
                println!(
                    "{:10},{:10},{:10},{:10},",
                    projection[2], projection[6], projection[10], projection[14]
                );
                println!(
                    "{:10},{:10},{:10},{:10}]",
                    projection[3], projection[7], projection[11], projection[15]
                );
            }

            let view_uniform_data = vs::ty::view_matrices {
                view: view.into(),
                projection: projection.into(),
            };

            let sub_buffer_view_uniforms = view_uniform_buffer_pool.next(view_uniform_data).unwrap();

            let layout0 = pipeline.descriptor_set_layout(0).unwrap();
            let set0 = Arc::new(
                PersistentDescriptorSet::start(layout0.clone())
                    .add_buffer(sub_buffer_view_uniforms.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );

            let lighting_uniform_data = fs::ty::light_parameters {
                view_position: camera_pos.into(),
                light: fs::ty::Light {
                    position: glm::vec3(2.0, 1.1, 0.0).into(),
                    ambient: glm::vec3(0.2, 0.2, 0.2).into(),
                    diffuse: glm::vec3(1.0, 1.0, 1.0).into(),
                    specular: glm::vec3(1.0, 1.0, 1.0).into(),
                    _dummy0: [0, 0, 0, 0],
                    _dummy1: [0, 0, 0, 0],
                    _dummy2: [0, 0, 0, 0]
                },
                material: fs::ty::Material {
                    ambient: glm::vec3(1.0, 1.0, 1.0).into(),
                    diffuse: glm::vec3(0.1, 0.3, 0.8).into(),
                    specular: glm::vec3(1.0, 1.0, 1.0).into(),
                    shininess: 20.0,
                    _dummy0: [0, 0, 0, 0],
                    _dummy1: [0, 0, 0, 0],
                },
                _dummy0: [0, 0, 0, 0],
                _dummy1: [0, 0, 0, 0],
            };

            let sub_buffer_lighting_uniforms = lighting_uniform_buffer_pool.next(lighting_uniform_data).unwrap();
            
            let layout1 = pipeline.descriptor_set_layout(1).unwrap();
            let set1 = Arc::new(
                PersistentDescriptorSet::start(layout1.clone())
                    .add_buffer(sub_buffer_lighting_uniforms.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );

            builder
                .begin_render_pass(framebuffers[image_num].clone(), false, clear_values)
                .unwrap();

            cube.draw(&mut builder, pipeline.clone(), set0.clone(), set1.clone())
                .expect("Could not add cube draw to cmd buffer");

            builder.end_render_pass().unwrap();

            let command_buffer = builder.build().unwrap();

            let future = previous_frame_end
                .take()
                .unwrap()
                .join(acquire_future)
                .then_execute(queue.clone(), command_buffer)
                .unwrap()
                .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
                .then_signal_fence_and_flush();

            match future {
                Ok(future) => {
                    previous_frame_end = Some(future.boxed());
                }
                Err(FlushError::OutOfDate) => {
                    recreate_swapchain = true;
                    previous_frame_end = Some(sync::now(device.clone()).boxed());
                }
                Err(e) => {
                    println!("Failed to flush future: {:?}", e);
                    previous_frame_end = Some(sync::now(device.clone()).boxed());
                }
            }
        }
        _ => (),
    });
}
