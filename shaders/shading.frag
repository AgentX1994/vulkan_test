#version 450

layout(location = 0) in vec3 f_position;
layout(location = 1) in vec3 f_normal;
layout(location = 2) in vec2 f_uv;

struct Light {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

layout(set = 1, binding = 0) uniform light_parameters {
    vec3 view_position;
    Light light;
};

layout(set = 2, binding = 0) uniform material_parameters {
    Material material;
};

layout(location = 0) out vec4 f_color;

void main() {
    // ambient
    vec3 ambient = light.ambient * material.ambient;

    // diffuse
    vec3 norm = normalize(f_normal);
    vec3 light_direction = normalize(light.position - f_position);
    float diff = max(dot(norm, light_direction), 0.0);
    vec3 diffuse = diff * light.diffuse * material.diffuse;

    // specular
    vec3 view_direction = normalize(view_position - f_position);
    vec3 reflect_direction = reflect(-light_direction, norm);
    float spec = pow(max(dot(view_direction, reflect_direction), 0.0), material.shininess);
    vec3 specular = spec * light.specular * material.specular;

    vec3 result = ambient + diffuse + specular;
    f_color = vec4(result, 1.0);
}