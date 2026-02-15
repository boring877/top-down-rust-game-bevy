use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}

impl Default for Collider {
    fn default() -> Self {
        Self { radius: 16.0 }
    }
}
