#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 uv;

layout(set = 0, binding = 0) uniform view_matrices {
    mat4 view;
    mat4 projection;
};

layout(location = 0) out vec3 f_position;
layout(location = 1) out vec3 f_normal;
layout(location = 2) out vec2 f_uv;

void main() {
    gl_Position = projection * view * vec4(position, 1.0);
    f_position = position;
    f_normal = normal;
    f_uv = uv;
}