use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d};

#[derive(Asset, TypePath, Debug, AsBindGroup, Clone)]
pub struct BossMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub hit_flash: f32,
    #[uniform(0)]
    pub health_percent: f32,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub is_moving: f32,
}

impl Material2d for BossMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/boss.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
