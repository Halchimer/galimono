#version 330 core

out vec4 final_color;

in vec2 TexCoord;

uniform sampler2D texture0;

void main() {
    final_color = texture(texture0, TexCoord);
}
