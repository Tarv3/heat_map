#version 400

uniform sampler2D map1;
uniform sampler2D map2;
uniform sampler2D bwmap;
uniform float contrast;
uniform vec2 min_pos;
uniform vec2 max_pos;

in vec2 f_position;
in vec2 f_tex_coord;

out vec4 colour;

vec3 hsv_to_rgb(in float hue) {
    float h = hue * 100;
    float x  = 1.0 - abs(mod(h / 20.0, 2.0) - 1.0);
    vec3 colour;
    if (h >= 0 && h < 20) {
        colour = vec3(1, x, 0);
    }
    else if (h >= 20 && h < 40) {
        colour = vec3(x, 1, 0);
    }
    else if (h >= 40 && h < 60) {
        colour = vec3(0, 1, x);
    }
    else if (h >= 60 && h < 80) {
        colour = vec3(0, x, 1);
    }
    else if (h >= 80 && h < 100) {
        colour = vec3(x, 0, 1);
    }
    else if (h > 100){
        colour = vec3(1, 0, 1);
    }
    else {
        colour = vec3(1, 0, 0);
    }
    return(colour);
}

void main() {
    ivec2 dims = textureSize(map1, 0);
    ivec2 tex_pos = ivec2(int(float(dims.x) * f_tex_coord.x), int(float(dims.y) * f_tex_coord.y));
    vec4 value1 = (texelFetch(map1, tex_pos, 0) - 0.5) * 2.0;

    dims = textureSize(map2, 0);
    tex_pos = ivec2(int(float(dims.x) * f_tex_coord.x), int(float(dims.y) * f_tex_coord.y));
    vec4 value2 = (texelFetch(map2, tex_pos, 0) - 0.5) * 2.0;

    float value = abs(value1.x - value2.x);

    float contrast_value = (259.0 * ( + 255.0))/(255.0 * (259.0 - contrast));
    float negative = min(0.0, value);
    float new_brightness = contrast_value * (value - 0.5) + 0.5;


    vec2 f_min_pos = (min_pos + vec2(180.0, 90)) / vec2(360, 180);
    vec2 f_max_pos = (max_pos + vec2(180.0, 90)) / vec2(360, 180);
    float tex_x = mix(f_min_pos.x, f_max_pos.x, f_tex_coord.x);
    float tex_y = mix(f_min_pos.y, f_max_pos.y, f_tex_coord.y);
    dims = textureSize(bwmap, 0);
    tex_pos = ivec2(int(float(dims.x) * tex_x), int(float(dims.y) * tex_y));

    colour = vec4(texelFetch(bwmap, tex_pos, 0).x * hsv_to_rgb(1 - new_brightness), 1.0);
    // colour = vec4(mix(hsv_to_rgb(1 - new_brightness), vec3(0.0, 0.0, 0.0), -min(0, value.x)), 1.0);

}