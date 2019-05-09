#version 140

in vec2 position;
in vec3 color;

uniform mat4 model;

out vec3 v_Color;

void main() {
    v_Color= color;
    gl_Position = model * vec4(position, 0.0, 1.0);
}