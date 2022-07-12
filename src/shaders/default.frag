#version 330 core

uniform sampler2DArray tiles; 

in vec2 v_tex_coord;

out vec4 color;

void main() {
    color = texture(tiles, vec3(v_tex_coord, 1));
}