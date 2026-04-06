uniform vec3 u_color;

varying vec3 vNormal;
varying vec3 vPosition;

void main() {
    // Simple lighting
    vec3 lightDir = normalize(vec3(1.0, 1.0, 1.0));
    float diff = max(dot(vNormal, lightDir), 0.0);
    
    // Cel shading - quantize the lighting
    float levels = 3.0;
    diff = floor(diff * levels) / levels;
    
    // Rim light
    vec3 viewDir = normalize(-vPosition);
    float rim = 1.0 - max(dot(vNormal, viewDir), 0.0);
    rim = pow(rim, 3.0);
    rim = step(0.3, rim); // Sharp rim
    
    vec3 finalColor = u_color * (0.2 + diff * 0.8) + vec3(rim * 0.3);
    
    gl_FragColor = vec4(finalColor, 1.0);
}
