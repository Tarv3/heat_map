#version 400

uniform sampler2D map;
uniform vec4 colour1;
uniform vec4 colour2;
uniform float contrast;

in vec2 f_position;
in vec2 f_tex_coord;

out vec4 colour;

void main() {
    ivec2 dims = textureSize(map, 0);
    ivec2 tex_pos = ivec2(int(float(dims.x) * f_tex_coord.x), int(float(dims.y) * f_tex_coord.y));
    vec4 value = (texelFetch(map, tex_pos, 0) - 0.5) * 2.0;
    float contrast_value = (259.0 * ( + 255.0))/(255.0 * (259.0 - contrast));
    float negative = min(0.0, value.x);
    float new_brightness = contrast_value * (value.x - 0.5) + 0.5;
    colour = mix(mix(vec4(0.0, 0.0, 1.0, 1.0), vec4(0.95, 0.28, 0.08, 1.0), new_brightness), vec4(0.0, 0.0, 0.0, 1.0), -negative);

}