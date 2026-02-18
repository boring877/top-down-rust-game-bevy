use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d};

#[derive(Asset, TypePath, Debug, AsBindGroup, Clone)]
pub struct BulletMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Material2d for BulletMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/bullet.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Asset, TypePath, Debug, AsBindGroup, Clone)]
pub struct BladeMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
    #[uniform(0)]
    pub progress: f32,
}

impl Material2d for BladeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/blade.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
