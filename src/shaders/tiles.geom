#version 330 core

uniform mat4 projection;

in VS_OUT {
    uint tile_id;
} gs_in[]; 

layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

out vec3 tex_coord;

void main() {
    uint tile_id = gs_in[0].tile_id;
    vec4 pos = gl_in[0].gl_Position;
    
    gl_Position = projection * pos;
    tex_coord = vec3(0.0, 0.0, tile_id); 
    EmitVertex();

    gl_Position = projection * (pos + vec4(1.0, 0.0, 0.0, 0.0));
    tex_coord = vec3(1.0, 0.0, tile_id);
    EmitVertex();
    
    gl_Position = projection * (pos + vec4(0.0, 1.0, 0.0, 0.0));
    tex_coord = vec3(0.0, 1.0, tile_id);
    EmitVertex();

    gl_Position = projection * (pos + vec4(1.0, 1.0, 0.0, 0.0));
    tex_coord = vec3(1.0, 1.0, tile_id);
    EmitVertex();

    EndPrimitive();
}  