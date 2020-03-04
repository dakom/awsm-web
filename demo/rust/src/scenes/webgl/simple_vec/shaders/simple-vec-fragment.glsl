precision mediump float;

uniform vec2 u_color[2]; 

void main() {
    gl_FragColor = vec4(u_color[0][0], u_color[0][1], u_color[1][0], u_color[1][1]); 
}
