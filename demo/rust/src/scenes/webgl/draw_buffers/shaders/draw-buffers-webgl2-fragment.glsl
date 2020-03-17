#version 300 es

precision highp float;

uniform vec4 u_color_visible; 
uniform vec4 u_color_hidden; 

layout(location = 0) out vec4 gbuf_hidden;
layout(location = 1) out vec4 gbuf_visible;

void main()
{
    gbuf_hidden = u_color_hidden; 
    gbuf_visible = u_color_visible;
}