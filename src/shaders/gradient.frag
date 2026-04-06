uniform float u_time;
uniform vec3 u_color;

varying vec2 vUv;
varying vec3 vPosition;

void main() {
    // Create animated gradient based on position and time
    float r = sin(vPosition.x * 2.0 + u_time) * 0.5 + 0.5;
    float g = sin(vPosition.y * 2.0 + u_time * 1.5) * 0.5 + 0.5;
    float b = sin(vPosition.z * 2.0 + u_time * 0.5) * 0.5 + 0.5;
    
    // Mix with base color
    vec3 gradient = vec3(r, g, b);
    vec3 finalColor = mix(u_color, gradient, 0.7);
    
    gl_FragColor = vec4(finalColor, 1.0);
}
