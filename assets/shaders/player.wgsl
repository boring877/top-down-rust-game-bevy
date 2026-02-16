// Modern Sci-Fi Soldier Player Shader with Animation
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct PlayerUniforms {
    color: vec4<f32>,
    hit_flash: f32,
    facing_angle: f32,
    is_moving: f32,
    time: f32,
}

@group(2) @binding(0) var<uniform> uniforms: PlayerUniforms;

fn sd_circle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}

fn sd_box(p: vec2<f32>, b: vec2<f32>) -> f32 {
    let d = abs(p) - b;
    return length(max(d, vec2<f32>(0.0))) + min(max(d.x, d.y), 0.0);
}

fn sd_rounded_box(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + r;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

fn sd_capsule(p: vec2<f32>, ra: f32, rb: f32, h: f32) -> f32 {
    let q = vec2<f32>(abs(p.x), p.y);
    let b = (ra - rb) / h;
    let a = sqrt(1.0 - b * b);
    let k = dot(q, vec2<f32>(-b, a));
    if (k < 0.0) { return length(q) - ra; }
    if (k > a * h) { return length(vec2<f32>(q.x, q.y - h)) - rb; }
    return dot(q, vec2<f32>(a, b)) - ra;
}

fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h = max(k - abs(a - b), 0.0) / k;
    return min(a, b) - h * h * k * 0.25;
}

fn rot(p: vec2<f32>, a: f32) -> vec2<f32> {
    let c = cos(a);
    let s = sin(a);
    return vec2<f32>(c * p.x - s * p.y, s * p.x + c * p.y);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let uv_flip = vec2<f32>(mesh.uv.x, 1.0 - mesh.uv.y);
    let uv = (uv_flip - center) * 2.0;
    
    // Animation
    let time = uniforms.time;
    let is_moving = uniforms.is_moving;
    
    // Idle breathing animation
    let breathe = sin(time * 2.0) * 0.01;
    
    // Walk cycle animation
    let walk = sin(time * 8.0) * is_moving;
    let arm_swing = walk * 0.15;
    let leg_swing = walk * 0.2;
    let body_bob = abs(sin(time * 16.0)) * is_moving * 0.02;
    
    // === HEAD - Angular helmet ===
    let head = sd_rounded_box(uv + vec2<f32>(0.0, -0.18 + body_bob), vec2<f32>(0.12, 0.11), 0.03);
    
    // Visor
    let visor = sd_box(uv + vec2<f32>(0.0, -0.17 + body_bob), vec2<f32>(0.09, 0.025));
    
    // === BODY - Armored torso ===
    let body = sd_rounded_box(uv + vec2<f32>(0.0, 0.02 + breathe + body_bob), vec2<f32>(0.12, 0.16), 0.04);
    
    // Core detail
    let core = sd_circle(uv + vec2<f32>(0.0, 0.0 + body_bob), 0.03);
    
    // === SHOULDERS - Armor pads ===
    let shoulder_l = sd_rounded_box(uv + vec2<f32>(0.18, -0.08 + body_bob), vec2<f32>(0.05, 0.06), 0.02);
    let shoulder_r = sd_rounded_box(uv + vec2<f32>(-0.18, -0.08 + body_bob), vec2<f32>(0.05, 0.06), 0.02);
    
    // === ARMS - With animation ===
    let arm_l = sd_capsule(rot(uv + vec2<f32>(0.2, 0.08 + body_bob), 0.2 - arm_swing), 0.025, 0.02, 0.14);
    let arm_r = sd_capsule(rot(uv + vec2<f32>(-0.2, 0.08 + body_bob), -0.2 + arm_swing), 0.025, 0.02, 0.14);
    
    // Hands
    let hand_l = sd_circle(uv + vec2<f32>(0.24, 0.22 + body_bob - arm_swing * 0.3), 0.03);
    let hand_r = sd_circle(uv + vec2<f32>(-0.24, 0.22 + body_bob + arm_swing * 0.3), 0.03);
    
    // === LEGS - With animation ===
    let leg_l = sd_capsule(rot(uv + vec2<f32>(-0.05, 0.24), leg_swing), 0.03, 0.025, 0.12);
    let leg_r = sd_capsule(rot(uv + vec2<f32>(0.05, 0.24), -leg_swing), 0.03, 0.025, 0.12);
    
    // Feet
    let foot_l = sd_rounded_box(uv + vec2<f32>(-0.08 + leg_swing * 0.3, 0.36), vec2<f32>(0.04, 0.015), 0.008);
    let foot_r = sd_rounded_box(uv + vec2<f32>(0.08 - leg_swing * 0.3, 0.36), vec2<f32>(0.04, 0.015), 0.008);
    
    // === GUN ===
    let gun = sd_box(uv + vec2<f32>(-0.32, 0.18 + body_bob + arm_swing * 0.2), vec2<f32>(0.08, 0.02));
    let gun_barrel = sd_box(uv + vec2<f32>(-0.42, 0.18 + body_bob + arm_swing * 0.2), vec2<f32>(0.03, 0.012));
    
    // === COMBINE ===
    var d = smin(head, body, 0.04);
    d = smin(d, shoulder_l, 0.02);
    d = smin(d, shoulder_r, 0.02);
    d = smin(d, arm_l, 0.03);
    d = smin(d, arm_r, 0.03);
    d = smin(d, hand_l, 0.02);
    d = smin(d, hand_r, 0.02);
    d = smin(d, leg_l, 0.03);
    d = smin(d, leg_r, 0.03);
    d = smin(d, foot_l, 0.02);
    d = smin(d, foot_r, 0.02);
    d = smin(d, gun, 0.01);
    d = smin(d, gun_barrel, 0.01);
    
    if (d > 0.01) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    
    // === COLORING ===
    let base_color = uniforms.color.rgb;
    
    // Dark armor
    let armor_dark = base_color * 0.35;
    let armor_mid = base_color * 0.6;
    let armor_light = base_color * 0.9;
    
    let edge = 1.0 - smoothstep(-0.015, 0.01, d);
    
    // Base armor
    var col = mix(armor_dark, armor_mid, 0.7);
    col = mix(col, armor_light, edge * 0.5);
    
    // Visor glow (cyan)
    let visor_mask = 1.0 - smoothstep(-0.01, 0.01, visor);
    let visor_col = vec3<f32>(0.2, 0.9, 1.0) * (1.0 + sin(time * 3.0) * 0.2);
    col = mix(col, visor_col, visor_mask * 0.85);
    
    // Core glow
    let core_mask = 1.0 - smoothstep(-0.02, 0.02, core);
    let core_col = vec3<f32>(0.3, 0.8, 1.0) * (1.0 + sin(time * 4.0) * 0.15);
    col = mix(col, core_col, core_mask * 0.6);
    
    // Gun metal
    let gun_mask = 1.0 - smoothstep(-0.005, 0.005, min(gun, gun_barrel));
    col = mix(col, vec3<f32>(0.2, 0.22, 0.25), gun_mask * 0.7);
    
    // Shoulder highlights
    let shoulder_mask = 1.0 - smoothstep(-0.01, 0.01, min(shoulder_l, shoulder_r));
    col = mix(col, armor_light, shoulder_mask * 0.3);
    
    // Hit flash
    col = mix(col, vec3<f32>(1.0, 0.5, 0.5), uniforms.hit_flash);
    
    let alpha = smoothstep(0.01, -0.01, d) * uniforms.color.a;
    
    return vec4<f32>(col, alpha);
}