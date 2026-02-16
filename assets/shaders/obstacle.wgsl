// Procedural obstacle shader with 3 shape types
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct ObstacleUniforms {
    color: vec4<f32>,
    shape_type: f32,
    _pad1: f32,
    _pad2: f32,
}

@group(2) @binding(0) var<uniform> uniforms: ObstacleUniforms;

// Noise function for variation
fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(hash(i), hash(i + vec2(1.0, 0.0)), u.x),
        mix(hash(i + vec2(0.0, 1.0)), hash(i + vec2(1.0, 1.0)), u.x),
        u.y
    );
}

// Rock shape (0) - irregular rounded blob
fn rock_shape(uv: vec2<f32>) -> f32 {
    var d = length(uv - 0.5) * 2.0;

    // Add noise-based distortion for rocky look
    let n1 = noise(uv * 4.0) * 0.15;
    let n2 = noise(uv * 8.0) * 0.08;
    d = d - n1 - n2;

    return 1.0 - smoothstep(0.7, 0.85, d);
}

// Crystal shape (1) - hexagonal/pointed
fn crystal_shape(uv: vec2<f32>) -> f32 {
    let centered = uv - 0.5;

    // Hexagonal shape
    let angle = atan2(centered.y, centered.x);
    let radius = length(centered) * 2.0;

    // Create 6 pointed star/crystal
    let hex_r = 0.4 + cos(angle * 6.0) * 0.08;
    let edge = 1.0 - smoothstep(hex_r - 0.1, hex_r, radius);

    // Add inner glow
    let inner = 1.0 - smoothstep(0.0, 0.3, radius);

    return max(edge, inner * 0.3);
}

// Pillar shape (2) - rectangular with rounded corners
fn pillar_shape(uv: vec2<f32>) -> f32 {
    let centered = uv - 0.5;
    let abs_c = abs(centered);

    // Rounded rectangle
    let corner_radius = 0.1;
    let rect_dist = max(abs_c.x - 0.35 + corner_radius, abs_c.y - 0.4 + corner_radius);
    let edge = 1.0 - smoothstep(0.0, corner_radius, rect_dist);

    // Add some texture
    let tex = noise(uv * 10.0) * 0.1;

    return edge + tex * edge;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;
    let shape_type = i32(uniforms.shape_type);

    // Get shape mask based on type
    var mask = 0.0;
    if (shape_type == 0) {
        mask = rock_shape(uv);
    } else if (shape_type == 1) {
        mask = crystal_shape(uv);
    } else {
        mask = pillar_shape(uv);
    }

    // Discard pixels outside shape
    if (mask < 0.01) {
        discard;
    }

    // Color with shading
    let base_color = uniforms.color.rgb;

    // Add depth shading based on distance from center
    let centered = uv - 0.5;
    let dist = length(centered);
    let shade = 1.0 - dist * 0.4;

    // Add highlight on one side
    let highlight = smoothstep(-0.1, 0.3, -centered.x - centered.y) * 0.3;

    let final_color = base_color * shade + highlight;

    return vec4<f32>(final_color, mask);
}
