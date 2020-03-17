#version 300 es

precision highp float;

in vec2 a_vertex;

uniform vec4 u_color_visible;
uniform vec4 u_color_hidden;
uniform mat4 u_modelViewProjection;
uniform mat4 u_size;

void main() {
    gl_Position = u_modelViewProjection * (u_size * vec4(a_vertex,0,1));
}
