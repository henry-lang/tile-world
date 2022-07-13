#version 330 core

layout (location = 0) in uint tile_id;

out VS_OUT {
    uint tile_id;
} vs_out; // Pass to fragment shader


void main() {
    int i = gl_VertexID;
    float x = float(i / 10);
    float y = float(i % 10);

    gl_Position = vec4(x, y, 0.0, 1.0);

    vs_out.tile_id = tile_id;
}