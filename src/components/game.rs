use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{AlphaMode2d, Material2d};

// ============================================================================
// ENTITIES
// ============================================================================

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct Boss;

// ============================================================================
// COMBAT - HEALTH
// ============================================================================

#[derive(Component)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    pub fn new(max: u32) -> Self {
        Self { current: max, max }
    }

    pub fn take_damage(&mut self, amount: u32) {
        self.current = self.current.saturating_sub(amount);
    }

    pub fn heal(&mut self, amount: u32) {
        self.current = self.current.saturating_add(amount);
    }

    pub fn is_dead(&self) -> bool {
        self.current == 0
    }
}

// ============================================================================
// COMBAT - BULLET
// ============================================================================

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
    pub damage: u32,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct SuperBullet;

#[derive(Component)]
pub struct FireRate {
    pub timer: Timer,
}

impl FireRate {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
        }
    }

    pub fn set_rate(&mut self, seconds: f32) {
        self.timer.set_duration(std::time::Duration::from_secs_f32(seconds));
    }
}

#[derive(Component)]
pub struct ShotCounter {
    pub count: u32,
}

impl Default for ShotCounter {
    fn default() -> Self {
        Self { count: 0 }
    }
}

#[derive(Component)]
pub struct DamageNumber {
    pub timer: Timer,
    pub damage: u32,
}

// ============================================================================
// SHADERS
// ============================================================================

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
