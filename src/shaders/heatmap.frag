varying vec2 vUv;
varying vec3 vPosition;

// Heat map color function
vec3 heatmap(float t) {
    t = clamp(t, 0.0, 1.0);
    
    vec3 a = vec3(0.0, 0.0, 0.5);    // Deep blue
    vec3 b = vec3(0.0, 1.0, 1.0);    // Cyan
    vec3 c = vec3(1.0, 1.0, 0.0);    // Yellow
    vec3 d = vec3(1.0, 0.0, 0.0);    // Red
    
    if (t < 0.33) {
        return mix(a, b, t * 3.0);
    } else if (t < 0.66) {
        return mix(b, c, (t - 0.33) * 3.0);
    } else {
        return mix(c, d, (t - 0.66) * 3.0);
    }
}

void main() {
    // Create heat pattern based on height
    float height = (vPosition.y + 1.0) * 0.5; // Normalize to 0-1
    
    // Add some pattern
    float pattern = sin(vPosition.x * 5.0) * sin(vPosition.z * 5.0) * 0.1;
    height += pattern;
    
    vec3 color = heatmap(height);
    
    gl_FragColor = vec4(color, 1.0);
}
