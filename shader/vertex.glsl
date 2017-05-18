#version 150 core

in vec2 position;
in vec4 colour;
in float radius;

out VertexData {
    vec4 colour;
	float radius;
} VertexOut;

void main() {
    gl_Position = vec4(position, 0, 1);
    VertexOut.colour = colour;
	VertexOut.radius = radius;
}
