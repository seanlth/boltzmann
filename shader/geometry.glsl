#version 150 core

layout (points) in;
layout (triangle_strip, max_vertices=4) out;

in VertexData {
    vec4 colour;
	float radius;
} VertexIn[];

out VertexData {
    vec4 colour;
    vec2 uv;
} VertexOut;

uniform float u_Aspect;


void main()
{
	float radius = VertexIn[0].radius;

    gl_Position = gl_in[0].gl_Position + vec4(-radius*u_Aspect, -radius, 0, 0);
    VertexOut.colour = VertexIn[0].colour;
    VertexOut.uv = vec2(-1, -1);
    EmitVertex();

    gl_Position = gl_in[0].gl_Position + vec4(radius*u_Aspect, -radius, 0, 0);
    VertexOut.colour = VertexIn[0].colour;
    VertexOut.uv = vec2(1, -1);
    EmitVertex();

    gl_Position = gl_in[0].gl_Position + vec4(-radius*u_Aspect, radius, 0, 0);
    VertexOut.colour = VertexIn[0].colour;
    VertexOut.uv = vec2(-1, 1);
    EmitVertex();

    gl_Position = gl_in[0].gl_Position + vec4(radius*u_Aspect, radius, 0, 0);
    VertexOut.colour = VertexIn[0].colour;
    VertexOut.uv = vec2(1, 1);
    EmitVertex();
}
