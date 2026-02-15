// Premium energy orb shader - 3D sphere illusion with lighting
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> color: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let center = vec2(0.5, 0.5);
    let uv = mesh.uv - center;

    // Create sphere normal from UV (3D illusion)
    let dist = length(uv);
    if (dist > 0.5) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0); // Outside sphere
    }

    // Calculate 3D normal for sphere surface
    let z = sqrt(max(0.0, 0.25 - dist * dist));
    let normal = vec3(uv.x, uv.y, z);

    // Light position (top-left-front for dramatic lighting)
    let light_dir = normalize(vec3(-0.3, 0.4, 0.8));

    // Diffuse lighting
    let diffuse = max(0.0, dot(normal, light_dir));

    // Specular highlight (rim light effect)
    let view_dir = vec3(0.0, 0.0, 1.0);
    let reflect_dir = reflect(-light_dir, normal);
    let specular = pow(max(0.0, dot(view_dir, reflect_dir)), 32.0);

    // Fresnel rim glow (edge lighting)
    let fresnel = pow(1.0 - max(0.0, dot(normal, view_dir)), 3.0);

    // Inner glow (subsurface scattering effect)
    let subsurface = pow(max(0.0, dot(-normal, light_dir)), 2.0) * 0.3;

    // Combine lighting
    let ambient = 0.15;
    let lighting = ambient + diffuse * 0.7 + subsurface;

    // Create hot core (bright center)
    let core_dist = length(uv) * 2.0;
    let core = pow(1.0 - core_dist, 3.0);

    // Mix base color with white hot core
    let hot_color = mix(color.rgb, vec3(1.0, 1.0, 1.0), core * 0.8);

    // Apply lighting to color
    let lit_color = hot_color * lighting;

    // Add specular highlight as bright white
    let final_color = lit_color + vec3(1.0, 1.0, 1.0) * specular * 0.8;

    // Add fresnel rim glow in the base color
    let rim_color = final_color + color.rgb * fresnel * 0.6;

    // Outer glow halo
    let halo = 1.0 - smoothstep(0.3, 0.5, dist);
    let alpha = halo * color.a;

    return vec4<f32>(rim_color, alpha);
}
