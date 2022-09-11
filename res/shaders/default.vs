#version 450
in vec2 position;
uniform mat4 projection;
out vec2 v_tex_coords;

void main() {
    if (gl_VertexID % 4 == 0) {
        v_tex_coords = vec2(0.0, 1.0);
    } else if (gl_VertexID % 4 == 1) {
        v_tex_coords = vec2(1.0, 1.0);
    } else if (gl_VertexID % 4 == 2) {
        v_tex_coords = vec2(0.0, 0.0);
    } else {
        v_tex_coords = vec2(1.0, 0.0);
    }
    gl_Position = projection * vec4(position, 0.0, 1.0);
}
