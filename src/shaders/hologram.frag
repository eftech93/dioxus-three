uniform float u_time;
uniform vec3 u_color;

varying vec3 vPosition;
varying vec3 vNormal;

void main() {
    // Scanline effect
    float scanline = step(0.8, sin(vPosition.y * 20.0 + u_time * 5.0));
    
    // Edge glow based on view angle
    float fresnel = 1.0 - abs(dot(vNormal, vec3(0.0, 0.0, 1.0)));
    fresnel = pow(fresnel, 2.0);
    
    // Flicker effect
    float flicker = sin(u_time * 10.0) * 0.05 + 0.95;
    
    // Combine effects
    vec3 hologramColor = u_color * (0.3 + scanline * 0.5) * flicker;
    hologramColor += u_color * fresnel * 0.8;
    
    float alpha = 0.6 + fresnel * 0.4;
    
    gl_FragColor = vec4(hologramColor, alpha);
}
