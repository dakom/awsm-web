precision mediump float;

uniform vec2 u_color[8]; 

void main() {
    gl_FragColor = vec4(u_color[0].x, u_color[0].y, u_color[2].x, u_color[2].y); 
}
