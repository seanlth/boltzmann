#version 150 core

layout (points) in;
layout (triangle_strip, max_vertices=4) out;

in VertexData {
    vec4 color;
} VertexIn[];

out VertexData {
    vec4 color;
    vec2 uv;
} VertexOut;

uniform Locals {
    float u_Aspect;
    float radius;
};


void main()
{
    gl_Position = gl_in[0].gl_Position + vec4(-radius*u_Aspect, -radius, 0, 0);
    VertexOut.color = VertexIn[0].color;
    VertexOut.uv = vec2(-1, -1);
    EmitVertex();

    gl_Position = gl_in[0].gl_Position + vec4(radius*u_Aspect, -radius, 0, 0);
    VertexOut.color = VertexIn[0].color;
    VertexOut.uv = vec2(1, -1);
    EmitVertex();

    gl_Position = gl_in[0].gl_Position + vec4(-radius*u_Aspect, radius, 0, 0);
    VertexOut.color = VertexIn[0].color;
    VertexOut.uv = vec2(-1, 1);
    EmitVertex();

    gl_Position = gl_in[0].gl_Position + vec4(radius*u_Aspect, radius, 0, 0);
    VertexOut.color = VertexIn[0].color;
    VertexOut.uv = vec2(1, 1);
    EmitVertex();
}
