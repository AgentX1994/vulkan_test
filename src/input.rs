use std::sync::{Arc, Mutex};

use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, MouseScrollDelta, WindowEvent};
use winit::event_loop::ControlFlow;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Input {
    pub resized: Option<PhysicalSize<u32>>,
    pub focused: bool,
    pub move_up_pressed: bool,
    pub move_down_pressed: bool,
    pub move_left_pressed: bool,
    pub move_right_pressed: bool,
    pub move_forward_pressed: bool,
    pub move_backward_pressed: bool,
    pub cursor_offset: (f64, f64),
    pub mouse_wheel_delta: f64,
    pub exiting: bool,
}

impl Default for Input {
    fn default() -> Input {
        Input {
            resized: None,
            focused: true,
            move_up_pressed: false,
            move_down_pressed: false,
            move_left_pressed: false,
            move_right_pressed: false,
            move_forward_pressed: false,
            move_backward_pressed: false,
            cursor_offset: (0.0, 0.0),
            mouse_wheel_delta: 0.0,
            exiting: false,
        }
    }
}

pub struct InputHandler {
    input: Input,
    should_request_exit: bool,
}

impl InputHandler {
    pub fn new() -> Arc<Mutex<InputHandler>> {
        Arc::new(Mutex::new(InputHandler {
            input: Input::default(),
            should_request_exit: false,
        }))
    }

    pub fn handle_event(&mut self, event: Event<()>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                self.input.exiting = true;
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Focused(is_focused) => self.input.focused = is_focused,
                WindowEvent::Resized(size) => self.input.resized = Some(size),
                _ => (),
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::Key(input) => {
                    match input.scancode {
                        1 => {
                            // escape
                            if self.input.focused {
                                self.input.exiting = true;
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                        17 => {
                            self.input.move_forward_pressed = input.state == ElementState::Pressed
                        } // w
                        30 => self.input.move_left_pressed = input.state == ElementState::Pressed, // a
                        31 => {
                            self.input.move_backward_pressed = input.state == ElementState::Pressed
                        } // s
                        32 => self.input.move_right_pressed = input.state == ElementState::Pressed, // d
                        57 => self.input.move_up_pressed = input.state == ElementState::Pressed, // space
                        42 => self.input.move_down_pressed = input.state == ElementState::Pressed, // shift
                        _ => (),
                    }
                }
                DeviceEvent::MouseMotion { delta } => {
                    self.input.cursor_offset.0 -= delta.0; // axes are reversed for some reason
                    self.input.cursor_offset.1 -= delta.1;
                }
                DeviceEvent::MouseWheel { delta } => match delta {
                    MouseScrollDelta::LineDelta(_x, y) => self.input.mouse_wheel_delta += y as f64,
                    MouseScrollDelta::PixelDelta(pos) => self.input.mouse_wheel_delta += pos.y,
                },
                _ => (),
            },

            _ => (),
        }
        if self.should_request_exit {
            self.input.exiting = true;
            *control_flow = ControlFlow::Exit;
        }
    }

    pub fn poll(&mut self) -> Input {
        let ret = self.input;
        self.input.cursor_offset = (0.0, 0.0); // reset offset
        self.input.mouse_wheel_delta = 0.0;
        ret
    }

    pub fn request_exit(&mut self) {
        self.should_request_exit = true;
    }
}
