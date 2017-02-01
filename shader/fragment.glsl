#version 150 core

in VertexData {
    vec4 colour;
    vec2 uv;
} VertexIn;

out vec4 Target0;

void main() {
    float alpha = ceil(max(1-dot(VertexIn.uv, VertexIn.uv), 0));
    Target0 = vec4(VertexIn.colour.xyz, VertexIn.colour.w*alpha);
}
