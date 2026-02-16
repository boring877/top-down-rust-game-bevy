// Flying energy blade projectile shader
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct BladeUniforms {
    color: vec4<f32>,
    progress: f32,
    _pad1: f32,
    _pad2: f32,
    _pad3: f32,
}

@group(2) @binding(0) var<uniform> uniforms: BladeUniforms;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let center = vec2(0.5, 0.5);
    let uv = mesh.uv - center;

    // Create elongated shape (blade pointing right/X direction)
    let stretched = vec2(uv.x * 0.6, uv.y * 2.0);
    let dist = length(stretched);

    // Distance from center for overall shape
    let circular_dist = length(uv);

    // Sharp blade core
    let core = 1.0 - smoothstep(0.0, 0.15, dist);

    // Inner glow layer
    let inner_glow = 1.0 - smoothstep(0.1, 0.35, dist);

    // Outer glow/aura
    let outer_glow = 1.0 - smoothstep(0.25, 0.5, circular_dist);

    // Combine layers for energy blade look
    let blade_intensity = core * 2.0 + inner_glow * 0.8 + outer_glow * 0.4;

    // Pulsing effect based on progress
    let pulse = 0.8 + 0.2 * sin(uniforms.progress * 6.28);

    // Hot white core, fading to blade color
    let hot_core = vec3(1.0, 1.0, 1.0);
    let base_color = uniforms.color.rgb;
    let blade_color = mix(base_color, hot_core, core * 0.7);

    // Apply intensity and pulse
    let final_color = blade_color * blade_intensity * pulse;

    // Alpha based on outer glow
    let alpha = outer_glow * uniforms.color.a;

    return vec4<f32>(final_color, alpha);
}
