// Modern clean floor shader
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct FloorUniforms {
    color: vec4<f32>,
    tile_size: f32,
    _pad1: f32,
    _pad2: f32,
}

@group(2) @binding(0) var<uniform> uniforms: FloorUniforms;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;

    // Grid pattern
    let grid_uv = uv * 16.0;
    let grid_id = floor(grid_uv);
    let grid_local = fract(grid_uv);

    // Thin grid lines
    let line_width = 0.03;
    let edge_x = min(grid_local.x, 1.0 - grid_local.x);
    let edge_y = min(grid_local.y, 1.0 - grid_local.y);
    let edge = min(edge_x, edge_y);

    // Base dark color
    let base_color = uniforms.color.rgb * 0.3;

    // Lighter tile color with subtle variation
    let tile_brightness = 0.9 + fract(sin(dot(grid_id, vec2(12.9898, 78.233))) * 43758.5453) * 0.2;
    let tile_color = uniforms.color.rgb * tile_brightness;

    // Blend between line and tile
    let t = smoothstep(0.0, line_width, edge);
    let color = mix(base_color, tile_color, t);

    // Subtle vignette from center
    let center_dist = length(uv - 0.5) * 1.2;
    let vignette = 1.0 - smoothstep(0.3, 0.8, center_dist) * 0.3;

    return vec4<f32>(color * vignette, 1.0);
}
