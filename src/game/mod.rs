mod boss;
mod bullet;
mod camera;
mod floor;
mod obstacle;
mod player;

use crate::game_state::GameState;
use bevy::prelude::*;
use boss::BossSpawnTimer;

pub use boss::*;
pub use bullet::*;
pub use camera::*;
pub use floor::*;
pub use obstacle::*;
pub use player::*;

pub fn game_plugin(app: &mut App) {
    app.init_resource::<BossSpawnTimer>()
    .add_systems(
        OnEnter(GameState::Game),
        (spawn_camera, spawn_floor, spawn_player, spawn_obstacles),
    )
    .add_systems(
        Update,
        (
            player_movement,
            clamp_player_to_arena,
            animate_player,
            update_camera,
            boss_chase_player,
            clamp_boss_to_arena,
            animate_boss,
            spawn_bullet,
            bullet_movement,
            bullet_collision,
            animate_damage_numbers,
            enemy_death,
            boss_attack,
            animate_blade,
            blade_collision,
            test_spawn_bosses,
        ).run_if(in_state(GameState::Game)),
    )
    .add_systems(
        OnExit(GameState::Game),
        (cleanup_player, cleanup_boss, cleanup_floor, cleanup_camera, cleanup_bullets, cleanup_obstacles),
    );
}
