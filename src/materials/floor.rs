use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d};

#[derive(Asset, TypePath, Debug, AsBindGroup, Clone)]
pub struct FloorMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub tile_size: f32,
    #[uniform(0)]
    pub _pad1: f32,
    #[uniform(0)]
    pub _pad2: f32,
}

impl Material2d for FloorMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/floor.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Opaque
    }
}
