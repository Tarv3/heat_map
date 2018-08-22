#version 400

uniform float contrast;

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
        colour = vec3(x * 0.75, 0, 1);
    }
    else if (h > 100){
        colour = vec3(0.75, 0, 1);
    }
    else {
        colour = vec3(1, 0, 0);
    }
    return(colour);
}

void main() {
    float contrast_value = (259.0 * ( + 255.0))/(255.0 * (259.0 - contrast));
    float new_brightness = contrast_value * (f_tex_coord.y - 0.5) + 0.5;
    colour = vec4(hsv_to_rgb(1 - new_brightness), 1.0);
}