use bevy::prelude::*;

// ============================================================================
// BULLET
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

// ============================================================================
// DAMAGE NUMBERS
// ============================================================================

#[derive(Component)]
pub struct DamageNumber {
    pub timer: Timer,
    pub damage: u32,
}

// ============================================================================
// BOSS ATTACKS
// ============================================================================

#[derive(Component)]
pub struct BladeAttack {
    pub timer: Timer,
    pub damage: u32,
    pub angle_start: f32,
    pub angle_end: f32,
    pub has_hit_player: bool,
    pub direction: Vec2,
    pub speed: f32,
}

#[derive(Component)]
pub struct BossAttackTimer {
    pub timer: Timer,
}

// ============================================================================
// ENEMY TAG
// ============================================================================

#[derive(Component)]
pub struct Enemy;
