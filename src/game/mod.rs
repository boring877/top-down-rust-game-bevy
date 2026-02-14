mod boss;
mod camera;
mod floor;
mod player;

use crate::game_state::GameState;
use bevy::prelude::*;

pub use boss::*;
pub use camera::*;
pub use floor::*;
pub use player::*;

pub fn game_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Game),
        (spawn_camera, spawn_floor, spawn_player, spawn_boss),
    )
    .add_systems(
        Update,
        (player_movement, update_camera, boss_chase_player).run_if(in_state(GameState::Game)),
    )
    .add_systems(
        OnExit(GameState::Game),
        (cleanup_player, cleanup_boss, cleanup_floor, cleanup_camera),
    );
}
