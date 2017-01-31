#version 150 core

in vec2 position;
in vec4 colour;

out VertexData {
    vec4 colour;
} VertexOut;

void main() {
    gl_Position = vec4(position, 0, 1);
    VertexOut.colour = colour;
}
