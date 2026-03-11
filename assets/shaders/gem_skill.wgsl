#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct GemSkillMaterial {
    color: vec4<f32>,
    time: f32,
    speed: f32,
    intensity: f32,
}

@group(2) @binding(0)
var<uniform> material: GemSkillMaterial;

fn hash(p: vec2<f32>) -> vec2<f32> {
    var p2 = vec2<f32>(dot(p, vec2<f32>(127.1, 311.7)), dot(p, vec2<f32>(269.5, 183.3)));
    return -1.0 + 2.0 * fract(sin(p2) * 43758.5453123);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(dot(hash(i + vec2<f32>(0.0, 0.0)), f - vec2<f32>(0.0, 0.0)),
            dot(hash(i + vec2<f32>(1.0, 0.0)), f - vec2<f32>(1.0, 0.0)), u.x),
        mix(dot(hash(i + vec2<f32>(0.0, 1.0)), f - vec2<f32>(0.0, 1.0)),
            dot(hash(i + vec2<f32>(1.0, 1.0)), f - vec2<f32>(1.0, 1.0)), u.x), u.y);
}

fn fbm(p: vec2<f32>) -> f32 {
    var v = 0.0;
    var a = 0.5;
    var shift = vec2<f32>(100.0);
    // Rotate to reduce axial bias
    let cos_val = cos(0.5);
    let sin_val = sin(0.5);
    let mat = mat2x2<f32>(cos_val, sin_val, -sin_val, cos_val);
    var p2 = p;
    for (var i = 0; i < 5; i = i + 1) {
        v += a * noise(p2);
        p2 = mat * p2 * 2.0 + shift;
        a *= 0.5;
    }
    return v;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    
    // Core center distance
    let dist = distance(uv, vec2<f32>(0.5, 0.5));
    
    // High quality time-based twisting
    let t = material.time * material.speed;
    let angle = atan2(uv.y - 0.5, uv.x - 0.5);
    
    // Create base fractal turbulence
    let q = vec2<f32>(fbm(uv * 5.0 + vec2<f32>(t * 0.2, t * 0.2)), fbm(uv * 5.0 - vec2<f32>(t * 0.2, t * 0.2)));
    let r = vec2<f32>(fbm(uv * 10.0 + q + vec2<f32>(t * 0.5, t * 0.5)), fbm(uv * 10.0 + q - vec2<f32>(t * 0.7, t * 0.7)));
    
    let f = fbm(uv * 15.0 + r + vec2<f32>(t, t));
    
    // Spiral waves mixed with the turbulence
    let spiral = sin(angle * 4.0 + t * 4.0 - dist * 15.0) * 0.5 + 0.5;
    
    let mixed_pattern = (f * 0.6 + spiral * 0.4);
    
    // Sharp pulsing core
    let core = smoothstep(0.3, 0.0, dist);
    
    // Flame-like edge glow ring
    let ring = smoothstep(0.45, 0.35, dist) * smoothstep(0.1, 0.35, dist);
    
    // Combine everything
    let final_intensity = (core + mixed_pattern * ring * 2.5) * material.intensity;
    
    let base_color = material.color.rgb;
    let highlight = vec3<f32>(1.0, 1.0, 1.0) * pow(final_intensity, 3.0);
    
    let final_color = mix(base_color * final_intensity * 1.5, base_color + highlight, clamp(final_intensity * 0.5, 0.0, 1.0));
    
    // Fade out smoothly at the edges
    let alpha = smoothstep(0.5, 0.4, dist) * material.color.a;
    
    return vec4<f32>(final_color, alpha * clamp(final_intensity * 1.5, 0.0, 1.0));
}
