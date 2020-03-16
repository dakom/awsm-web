#extension GL_EXT_draw_buffers : require 

precision highp float; 

uniform vec4 u_color_visible; 
uniform vec4 u_color_hidden; 

void main() {

    gl_FragData[0] = u_color_hidden; 
    gl_FragData[1] = u_color_visible;
}
