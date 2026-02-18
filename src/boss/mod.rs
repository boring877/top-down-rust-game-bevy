mod spawn;
mod ai;
mod attack;
mod death;
pub mod particles;

pub use spawn::*;
pub use ai::*;
pub use attack::*;
pub use death::*;
pub use particles::*;

use bevy::prelude::*;

#[derive(Resource)]
pub struct BossSpawnTimer {
    pub timer: Timer,
}

impl Default for BossSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(crate::constants::BOSS_SPAWN_INTERVAL, TimerMode::Repeating),
        }
    }
}

// ============================================================================
// BOSS DEATH ANIMATION COMPONENTS
// ============================================================================

#[derive(Component)]
pub struct Dying {
    pub timer: Timer,
    pub stage: DeathStage,
    pub original_scale: Vec3,
    pub electric_entity: Option<Entity>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DeathStage {
    Collapsing,  // Shrinking + electricity
    Exploding,   // Final explosion
}
