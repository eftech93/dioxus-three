uniform float u_time;

varying vec2 vUv;
varying float vElevation;

void main() {
    vUv = uv;
    
    // Create wave displacement
    vec3 pos = position;
    float elevation = sin(pos.x * 4.0 + u_time) * 0.1;
    elevation += sin(pos.y * 3.0 + u_time * 0.8) * 0.1;
    elevation += sin((pos.x + pos.y) * 2.0 + u_time * 1.5) * 0.05;
    
    pos.z += elevation;
    vElevation = elevation;
    
    gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
}
