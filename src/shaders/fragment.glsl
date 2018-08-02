#version 400

uniform sampler2D map;
uniform vec4 colour1;
uniform vec4 colour2;

in vec2 f_position;
in vec2 f_tex_coord;

out vec4 colour;

void main() {
    vec4 red = textureGather(map, f_tex_coord, 0);
    vec4 green = textureGather(map, f_tex_coord, 1);
    vec4 blue = textureGather(map, f_tex_coord, 2);
    colour = mix(colour1, colour2, red.x);
}