use std::sync::{Arc, Mutex};

use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{Event, KeyboardInput, WindowEvent};
use winit::event_loop::ControlFlow;

#[derive(Clone, Copy, Debug)]
pub struct Input {
    pub resized: Option<PhysicalSize<u32>>,
    pub move_up_pressed: bool,
    pub move_down_pressed: bool,
    pub move_left_pressed: bool,
    pub move_right_pressed: bool,
    pub move_forward_pressed: bool,
    pub move_backward_pressed: bool,
    pub cursor_position: PhysicalPosition<f64>,
}

impl Default for Input {
    fn default() -> Input {
        Input {
            resized: None,
            move_up_pressed: false,
            move_down_pressed: false,
            move_left_pressed: false,
            move_right_pressed: false,
            move_forward_pressed: false,
            move_backward_pressed: false,
            cursor_position: PhysicalPosition::new(0.0, 0.0),
        }
    }
}

pub struct InputHandler {
    input: Input,
}

impl InputHandler {
    pub fn new() -> Arc<Mutex<InputHandler>> {
        Arc::new(Mutex::new(InputHandler {
            input: Input::default(),
        }))
    }

    pub fn handle_event(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => self.input.resized = Some(size),
                WindowEvent::KeyboardInput { input, .. } => println!("Input {:?}", input),
                WindowEvent::CursorMoved { position, .. } => self.input.cursor_position = position,
                _ => (),
            },
            _ => (),
        }
    }

    pub fn poll(&mut self) -> Input {
        let last_input = self.input;
        self.input = Input::default();
        self.input.cursor_position = last_input.cursor_position; // prevent mouse from jumping
        last_input
    }
}
