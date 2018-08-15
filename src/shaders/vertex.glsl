#version 140

in vec2 position;
in vec2 tex_coord;

out vec2 f_position;
out vec2 f_tex_coord;

void main() {
    f_position = position;
    f_tex_coord = tex_coord;
    //gl_Position = vec4(position.y / 180.0, position.x / 90.0, 0.0, 1.0);
    gl_Position = vec4(position, 0.0, 1.0);
}