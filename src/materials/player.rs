use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d};

#[derive(Asset, TypePath, Debug, AsBindGroup, Clone)]
pub struct PlayerMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub hit_flash: f32,
    #[uniform(0)]
    pub facing_angle: f32,
    #[uniform(0)]
    pub is_moving: f32,
    #[uniform(0)]
    pub time: f32,
}

impl Material2d for PlayerMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/player.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
