#version 450

layout(location = 0) in vec3 f_position;
layout(location = 1) in vec3 f_normal;
layout(location = 2) in vec2 f_uv;
layout(location = 0) out vec4 f_color;

const float left_side = -2.0;
const float right_side = 1.0;
const float bottom_side = -1.0;
const float top_side = 1.0;

vec2 map_uv_to_coords(vec2 uv) {
    // maps (0.0, 0.0) -> (1.0, 1.0)
    // to (left_side, bottom_side) -> (right_side, top_side)
    float u_prime = uv.x*(right_side - left_side) + left_side;
    float v_prime = uv.y*(top_side - bottom_side) + bottom_side;
    return vec2(u_prime, v_prime);
}

void main() {
    //f_color = vec4(f_uv, 0.0, 1.0);
    //return;
    vec2 c = map_uv_to_coords(f_uv);
    vec2 z = vec2(0.0, 0.0);
    float i;
    for (i = 0.0; i < 1.0; i+= 0.005) {
        z = vec2(
            z.x * z.x - z.y * z.y + c.x,
            z.y * z.x + z.x * z.y + c.y
        );

        if (length(z) > 4.0) {
            break;
        }
    }

    f_color = vec4(vec3(i), 1.0);
}