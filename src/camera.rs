use nalgebra_glm as glm;

use crate::utility::{clamp, radians};

pub enum CameraMoveDirection {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[derive(Clone, Debug)]
pub struct Camera {
    position: glm::Vec3,
    forward: glm::Vec3,
    up: glm::Vec3,
    right: glm::Vec3,
    world_up: glm::Vec3,
    yaw: f32,
    pitch: f32,
    speed: f32,
    mouse_sensitivity: f32,
    zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        let mut cam = Camera {
            position: glm::vec3(0.0, 0.0, 0.0),
            forward: glm::vec3(0.0, 0.0, -1.0),
            up: glm::vec3(0.0, -11.0, 0.0),
            right: glm::vec3(1.0, 0.0, 0.0),
            world_up: glm::vec3(0.0, -1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            speed: 2.5,
            mouse_sensitivity: 0.1,
            zoom: 45.0,
        };

        cam.update_camera_vectors();
        cam
    }
}

impl Camera {
    pub fn new(position: glm::Vec3, up: glm::Vec3, yaw: f32, pitch: f32) -> Self {
        let mut camera = Camera::default();
        camera.position = position;
        camera.world_up = up;
        // Fix for vulkan's reversed y axis
        camera.world_up[1] *= -1.0;
        camera.yaw = yaw;
        camera.pitch = pitch;
        camera.update_camera_vectors();
        camera
    }

    pub fn position(&self) -> glm::Vec3 {
        self.position
    }

    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::look_at(&self.position, &(self.position + self.forward), &self.up)
    }

    pub fn move_camera(&mut self, direction: CameraMoveDirection, delta_time: f32) {
        let velocity = self.speed * delta_time;
        match direction {
            CameraMoveDirection::FORWARD => self.position += self.forward * velocity,
            CameraMoveDirection::BACKWARD => self.position -= self.forward * velocity,
            CameraMoveDirection::RIGHT => self.position += self.right * velocity,
            CameraMoveDirection::LEFT => self.position -= self.right * velocity,
            // Up and down are reversed, since the up vector is reversed in vulkan
            CameraMoveDirection::UP => self.position -= self.up * velocity,
            CameraMoveDirection::DOWN => self.position += self.up * velocity,
        }
    }

    pub fn turn_camera(&mut self, x_offset: f32, y_offset: f32) {
        self.yaw += x_offset * self.mouse_sensitivity;
        self.pitch += y_offset * self.mouse_sensitivity;

        self.pitch = clamp(self.pitch, -89.0, 89.0);

        self.update_camera_vectors();
    }

    pub fn zoom_camera(&mut self, offset: f32) {
        self.zoom -= offset;
        self.zoom = clamp(self.zoom, 1.0, 45.0);
    }

    pub fn zoom(&self) -> f32 {
        radians(self.zoom)
    }

    fn update_camera_vectors(&mut self) {
        let forward = glm::vec3(
            radians(self.yaw).cos() * radians(self.pitch).cos(),
            radians(self.pitch).sin(),
            radians(self.yaw).sin() * radians(self.pitch).cos(),
        );
        self.forward = glm::normalize(&forward);
        self.right = glm::normalize(&glm::cross(&self.forward, &self.world_up));
        self.up = glm::normalize(&glm::cross(&self.right, &self.forward));
    }
}
