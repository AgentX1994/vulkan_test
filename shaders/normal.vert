#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 uv;

layout(set = 0, binding = 0) uniform view_matrices {
    mat4 view;
    mat4 projection;
};

layout(set = 1, binding = 0) uniform world_matrix {
    mat4 world;
};

layout(location = 0) out vec3 f_position;
layout(location = 1) out vec3 f_normal;
layout(location = 2) out vec2 f_uv;

void main() {
    vec4 world_position = world * vec4(position, 1.0);
    gl_Position = projection * view * world_position;
    f_position = vec3(world_position);
    f_normal = vec3(inverse(transpose(world)) * vec4(normal, 0.0));
    f_uv = uv;
}