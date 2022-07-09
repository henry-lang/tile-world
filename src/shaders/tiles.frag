#version 430 core

layout(location = 0) out vec4 color;

void main() {
    vec2 q = gl_FragCoord.xy / 1024.;
    color = vec4(q, 1., 1.);
}