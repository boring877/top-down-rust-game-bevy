use bevy::prelude::*;

// ============================================================================
// ENTITIES
// ============================================================================

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct Boss;

#[derive(Component)]
pub struct BossLastPosition(pub Vec2);

#[derive(Component)]
pub struct Obstacle;

#[derive(Clone, Copy, PartialEq)]
pub enum ObstacleShape {
    Rock,
    Crystal,
    Pillar,
}

#[derive(Component)]
pub struct Floor;

// ============================================================================
// HEALTH
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
