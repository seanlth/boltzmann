#version 150 core

in VertexData {
    vec4 colour;
} VertexIn;

out vec4 colour;

void main() {
    colour = VertexIn.colour;
}
