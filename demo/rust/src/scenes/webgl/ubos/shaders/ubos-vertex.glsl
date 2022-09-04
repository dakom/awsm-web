#version 300 es

precision mediump float;

in vec3 a_vertex;
in vec4 a_color;

out vec4 v_color;

layout (std140) uniform camera {
    mat4 u_view;
    mat4 u_projection;
} ubo_camera;

layout (std140) uniform model {
    mat4 u_size;
    mat4 u_model;
} ubo_model;


layout (std140) uniform scale {
    float u_scale_x;
    float u_scale_y;
    float u_scale_z;
    float u_scale_w; // needed for layout padding
} ubo_scale;

void main() {
    mat4 scale_mat = mat4(1.0);
    scale_mat[0][0] = ubo_scale.u_scale_x;
    scale_mat[1][1] = ubo_scale.u_scale_y;
    scale_mat[2][2] = ubo_scale.u_scale_z;
    mat4 size = ubo_model.u_size * scale_mat;

    //u_size[
    mat4 mvp = (ubo_camera.u_projection * (ubo_camera.u_view * ubo_model.u_model));
    gl_Position = mvp * (size * vec4(a_vertex,1));
    v_color = a_color;
}
