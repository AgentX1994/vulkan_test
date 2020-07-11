use std::error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::context::RenderContext;
use crate::input::InputHandler;
use crate::material::phong::Phong;
use crate::material::Material;
use crate::mesh::cube::Cube;
use crate::renderer::Renderer;
use crate::window::RenderWindow;

use nalgebra_glm as glm;
use vulkano::buffer::cpu_pool::CpuBufferPool;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::{DeviceExtensions, Features};
use vulkano::swapchain::Surface;
use vulkano::sync::GpuFuture;

use winit::window::Window;

pub struct Controller {
    context: Arc<RenderContext>,
    surface: Arc<Surface<Window>>,
    input_handler: Arc<Mutex<InputHandler>>,
    running: bool,
}

impl Controller {
    pub fn start() -> Result<(), Box<dyn error::Error + Send + Sync>> {
        let app_info = vulkano::app_info_from_cargo_toml!();

        let context = RenderContext::new(
            Some(&app_info),
            &Features::none(),
            &vulkano_win::required_extensions(),
            &DeviceExtensions {
                khr_storage_buffer_storage_class: true,
                khr_swapchain: true,
                ..DeviceExtensions::none()
            },
            vec![],
        )?;

        let window = RenderWindow::new(context.instance());
        let surface = window.surface();

        let controller = Controller {
            context,
            surface,
            input_handler: InputHandler::new(),
            running: false,
        };
        controller.run(window)
    }

    fn run(self, window: RenderWindow) -> Result<(), Box<dyn error::Error + Send + Sync>> {
        let input_handler_clone = self.input_handler.clone();
        let t = thread::spawn(move || self.run_internal());

        window.run_event_loop(move |e, _, c| {
            input_handler_clone
                .lock()
                .expect("Unable to lock input mutex")
                .handle_event(e, c);
        });

        // Make type checker happy, even if we shouldn't get here
        t.join().unwrap()
    }

    fn run_internal(mut self) -> Result<(), Box<dyn error::Error + Send + Sync>> {
        let queue = self.context.queue();

        let mut renderer = Renderer::new(self.context.clone(), self.surface.clone())?;

        let (phong_material, future) = Phong::new(
            glm::vec3(1.0, 1.0, 1.0),
            glm::vec3(0.1, 0.4, 0.8),
            glm::vec3(1.0, 1.0, 1.0),
            self.context.device(),
            queue.clone(),
            renderer.render_pass(),
        )?;

        let cube = Cube::new(self.context.device(), phong_material.clone());

        // for timing
        let start_time = Instant::now();

        // for uniforms
        let view_uniform_buffer_pool = CpuBufferPool::uniform_buffer(self.context.device());
        let lighting_uniform_buffer_pool = CpuBufferPool::uniform_buffer(self.context.device());

        let mut aspect_ratio = 1280.0f32 / 1024.0f32;

        self.running = true;
        let mut previous_frame_end = Some(future.boxed());
        while self.running {
            let input = self
                .input_handler
                .lock()
                .expect("could not lock input handler")
                .poll();

            if let Some(size) = input.resized {
                aspect_ratio = size.width as f32 / size.height as f32;
                renderer.resized();
            }

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

            let view_uniform_data = crate::material::phong::vs::ty::view_matrices {
                view: view.into(),
                projection: projection.into(),
            };

            let sub_buffer_view_uniforms =
                view_uniform_buffer_pool.next(view_uniform_data).unwrap();

            let camera_layout = phong_material.get_view_layout();
            let camera_set = Arc::new(
                PersistentDescriptorSet::start(camera_layout.clone())
                    .add_buffer(sub_buffer_view_uniforms.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );

            let lighting_uniform_data = crate::material::phong::fs::ty::light_parameters {
                view_position: camera_pos.into(),
                light: crate::material::phong::fs::ty::Light {
                    position: glm::vec3(2.0, 1.1, 0.0).into(),
                    ambient: glm::vec3(0.2, 0.2, 0.2).into(),
                    diffuse: glm::vec3(1.0, 1.0, 1.0).into(),
                    specular: glm::vec3(1.0, 1.0, 1.0).into(),
                    _dummy0: [0, 0, 0, 0],
                    _dummy1: [0, 0, 0, 0],
                    _dummy2: [0, 0, 0, 0],
                },
                _dummy0: [0, 0, 0, 0],
            };

            let sub_buffer_lighting_uniforms = lighting_uniform_buffer_pool
                .next(lighting_uniform_data)
                .unwrap();

            let lighting_layout = phong_material.get_lighting_layout();
            let lighting_set = Arc::new(
                PersistentDescriptorSet::start(lighting_layout.clone())
                    .add_buffer(sub_buffer_lighting_uniforms.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            );

            previous_frame_end =
                renderer.render(&cube, camera_set, lighting_set, previous_frame_end);
        }

        Ok(())
    }
}
