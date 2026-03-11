use bevy::prelude::*;
use bevy::sprite_render::{Material2d, AlphaMode2d};
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GemSkillMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub speed: f32,
    #[uniform(0)]
    pub intensity: f32,
}

impl Material2d for GemSkillMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/gem_skill.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
