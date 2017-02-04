#version 150 core

in VertexData {
    vec4 colour;
    vec2 uv;
} VertexIn;

out vec4 colour;

void main() {
    colour = VertexIn.colour;
}
