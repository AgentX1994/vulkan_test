use std::f32::consts::PI;

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    x.max(min).min(max)
}

pub fn radians(degrees: f32) -> f32 {
    degrees * PI / 180.0f32
}
