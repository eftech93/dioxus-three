uniform float u_time;
uniform vec3 u_color;

varying vec2 vUv;
varying float vElevation;

void main() {
    // Water color based on elevation
    vec3 deepColor = u_color * 0.5;
    vec3 surfaceColor = u_color * 1.5;
    
    float mixStrength = (vElevation + 0.25) * 2.0;
    vec3 color = mix(deepColor, surfaceColor, mixStrength);
    
    // Add foam at peaks
    float foam = step(0.18, vElevation);
    color = mix(color, vec3(1.0), foam * 0.5);
    
    gl_FragColor = vec4(color, 0.9);
}
