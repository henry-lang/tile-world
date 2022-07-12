#version 330 core

layout (location = 0) in vec2 tex_coord;
layout (location = 1) in vec2 position;

out vec2 v_tex_coord; // Pass to fragment shader

uniform mat4 projection;

void main() {
    gl_Position = projection * vec4(position, 0.0, 1.0);

    v_tex_coord = tex_coord;
}