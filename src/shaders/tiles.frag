#version 330 core

uniform sampler2DArray tiles; 

in vec3 tex_coord;

out vec4 color;

void main() {
    color = texture(tiles, tex_coord);
}