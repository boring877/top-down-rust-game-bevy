use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d};

#[derive(Asset, TypePath, Debug, AsBindGroup, Clone)]
pub struct ObstacleMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub shape_type: f32,
    #[uniform(0)]
    pub _pad1: f32,
    #[uniform(0)]
    pub _pad2: f32,
}

impl Material2d for ObstacleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/obstacle.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
