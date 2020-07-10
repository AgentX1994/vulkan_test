#version 450

layout(location = 0) in vec3 f_position;
layout(location = 1) in vec3 f_normal;
layout(location = 2) in vec2 f_uv;

layout(location = 0) out vec4 f_color;

void main() {
    // Map the normal into the range 0-1
    f_color = vec4(abs(f_normal), 1.0);
}