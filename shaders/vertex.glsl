#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 tex_coords;

out vec2 TexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    TexCoord = tex_coords;
}
