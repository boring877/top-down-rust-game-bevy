// Boss Shader - Full animation (idle + walk)
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct BossUniforms {
    color: vec4<f32>,
    hit_flash: f32,
    health_percent: f32,
    time: f32,
    is_moving: f32,
}

@group(2) @binding(0) var<uniform> uniforms: BossUniforms;

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
    
    let time = uniforms.time;
    let health = uniforms.health_percent;
    let rage = 1.0 - health;
    let is_moving = uniforms.is_moving;
    
    // === ANIMATIONS ===
    // Idle animations (always active)
    let breathe = sin(time * 1.5) * 0.015 * (1.0 - is_moving);
    let hover = sin(time * 2.0) * 0.008;
    let arm_idle = sin(time * 2.5) * 0.06 * (1.0 - is_moving);
    
    // Walk animations (only when moving)
    let walk_speed = 6.0;
    let walk = sin(time * walk_speed) * is_moving;
    let arm_swing = walk * 0.25;
    let leg_swing = walk * 0.3;
    let body_bob = abs(sin(time * walk_speed * 2.0)) * is_moving * 0.03;
    let body_sway = sin(time * walk_speed) * is_moving * 0.015;
    
    // === BODY ===
    let body = sd_rounded_box(uv + vec2<f32>(body_sway, hover + breathe + body_bob), vec2<f32>(0.22, 0.28), 0.06);
    let chest = sd_rounded_box(uv + vec2<f32>(body_sway, -0.05 + hover + breathe + body_bob), vec2<f32>(0.14, 0.12), 0.03);
    
    // === HELMET ===
    let helmet = sd_rounded_box(uv + vec2<f32>(body_sway * 0.5, -0.35 + hover + body_bob * 0.5), vec2<f32>(0.12, 0.1), 0.03);
    let face = sd_rounded_box(uv + vec2<f32>(body_sway * 0.5, -0.34 + hover + body_bob * 0.5), vec2<f32>(0.09, 0.06), 0.02);
    
    // Eyes with pulse
    let eye_pulse = 1.0 + sin(time * 3.0 + rage * 5.0) * 0.2;
    let eye_l = sd_circle(uv + vec2<f32>(-0.045 + body_sway * 0.5, -0.33 + hover + body_bob * 0.5), 0.018 * eye_pulse);
    let eye_r = sd_circle(uv + vec2<f32>(0.045 + body_sway * 0.5, -0.33 + hover + body_bob * 0.5), 0.018 * eye_pulse);
    
    // === HORNS ===
    let horn_sway = sin(time * 1.0) * 0.02 * (1.0 - is_moving);
    let horn_l = sd_capsule(rot(uv + vec2<f32>(0.1 + body_sway * 0.3, 0.45 + hover), -0.35 + horn_sway), 0.02, 0.008, 0.14);
    let horn_r = sd_capsule(rot(uv + vec2<f32>(-0.1 + body_sway * 0.3, 0.45 + hover), 0.35 - horn_sway), 0.02, 0.008, 0.14);
    
    // === SHOULDERS ===
    let shoulder_l = sd_rounded_box(uv + vec2<f32>(0.32 + body_sway * 0.3, -0.1 + hover + body_bob), vec2<f32>(0.1, 0.08), 0.03);
    let shoulder_r = sd_rounded_box(uv + vec2<f32>(-0.32 + body_sway * 0.3, -0.1 + hover + body_bob), vec2<f32>(0.1, 0.08), 0.03);
    let spike_l = sd_capsule(uv + vec2<f32>(0.4 + body_sway * 0.3, -0.1 + hover), 0.015, 0.006, 0.1);
    let spike_r = sd_capsule(uv + vec2<f32>(-0.4 + body_sway * 0.3, -0.1 + hover), 0.015, 0.006, 0.1);
    
    // === ARMS - walk + idle animation ===
    let arm_l = sd_capsule(rot(uv + vec2<f32>(0.35 + body_sway * 0.2, 0.1 + hover + body_bob), 0.1 + arm_idle + arm_swing), 0.035, 0.025, 0.18);
    let arm_r = sd_capsule(rot(uv + vec2<f32>(-0.35 + body_sway * 0.2, 0.1 + hover + body_bob), -0.1 - arm_idle - arm_swing), 0.035, 0.025, 0.18);
    
    // === LEGS - walk animation ===
    let leg_l = sd_capsule(rot(uv + vec2<f32>(-0.1 + body_sway, 0.38 + hover), leg_swing), 0.04, 0.03, 0.16);
    let leg_r = sd_capsule(rot(uv + vec2<f32>(0.1 + body_sway, 0.38 + hover), -leg_swing), 0.04, 0.03, 0.16);
    
    // === CORE (weak point) - pulsing ===
    let core_pulse = 1.0 + sin(time * 4.0) * 0.15;
    let core = sd_circle(uv + vec2<f32>(body_sway, body_bob), 0.04 * core_pulse);
    
    // === BELT ===
    let belt = sd_box(uv + vec2<f32>(body_sway, 0.18 + hover + body_bob), vec2<f32>(0.16, 0.025));
    let buckle = sd_box(uv + vec2<f32>(body_sway, 0.18 + hover + body_bob), vec2<f32>(0.03, 0.035));
    
    // === COMBINE ===
    var d = smin(body, helmet, 0.04);
    d = smin(d, horn_l, 0.02);
    d = smin(d, horn_r, 0.02);
    d = smin(d, shoulder_l, 0.03);
    d = smin(d, shoulder_r, 0.03);
    d = smin(d, spike_l, 0.02);
    d = smin(d, spike_r, 0.02);
    d = smin(d, arm_l, 0.03);
    d = smin(d, arm_r, 0.03);
    d = smin(d, leg_l, 0.03);
    d = smin(d, leg_r, 0.03);
    
    if (d > 0.01) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    
    // === COLORING ===
    let base = uniforms.color.rgb;
    
    // Dark armor
    let armor_dark = vec3<f32>(0.25, 0.12, 0.12);
    let armor_mid = base * 0.6;
    let armor_light = base * 0.9;
    
    let edge = 1.0 - smoothstep(-0.015, 0.01, d);
    
    // Base armor color
    var col = mix(armor_dark, armor_mid, 0.6);
    col = mix(col, armor_light, edge * 0.5);
    
    // Rage tint (more red when damaged)
    col = mix(col, vec3<f32>(0.5, 0.1, 0.1), rage * 0.4);
    
    // Face plate (darker)
    let face_mask = 1.0 - smoothstep(-0.01, 0.01, face);
    col = mix(col, armor_dark * 0.7, face_mask * 0.6);
    
    // Glowing eyes
    let eye_glow = 1.0 - smoothstep(-0.01, 0.01, min(eye_l, eye_r));
    let eye_col = vec3<f32>(1.0, 0.2, 0.1) * (1.0 + rage * 0.5 + sin(time * 3.0) * 0.2);
    col = mix(col, eye_col, eye_glow * 0.7);
    
    // Core glow (pulsing)
    let core_glow = 1.0 - smoothstep(-0.03, 0.02, core);
    let core_col = vec3<f32>(0.8, 0.25, 0.15) * (1.0 + rage * 0.3 + sin(time * 4.0) * 0.2);
    col = mix(col, core_col, core_glow * 0.5);
    
    // Belt highlight
    let belt_glow = 1.0 - smoothstep(-0.005, 0.005, belt);
    col = mix(col, armor_light * 0.8, belt_glow * 0.3);
    
    // Buckle
    let buckle_glow = 1.0 - smoothstep(-0.005, 0.005, buckle);
    col = mix(col, vec3<f32>(0.6, 0.5, 0.3), buckle_glow * 0.5);
    
    // Hit flash
    col = mix(col, vec3<f32>(1.0, 0.6, 0.6), uniforms.hit_flash);
    
    let alpha = smoothstep(0.01, -0.01, d) * uniforms.color.a;
    
    return vec4<f32>(col, alpha);
}