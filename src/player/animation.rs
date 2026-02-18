use crate::components::{Player, PlayerAction};
use crate::materials::PlayerMaterial;
use bevy::prelude::*;
use bevy::sprite_render::MeshMaterial2d;
use leafwing_input_manager::prelude::*;

pub fn animate_player(
    time: Res<Time>,
    mut materials: ResMut<Assets<PlayerMaterial>>,
    players: Query<(&ActionState<PlayerAction>, &MeshMaterial2d<PlayerMaterial>), With<Player>>,
) {
    for (action_state, material_handle) in players.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.time += time.delta_secs();

            let is_moving = action_state.pressed(&PlayerAction::MoveUp)
                || action_state.pressed(&PlayerAction::MoveDown)
                || action_state.pressed(&PlayerAction::MoveLeft)
                || action_state.pressed(&PlayerAction::MoveRight);

            material.is_moving = if is_moving { 1.0 } else { 0.0 };
        }
    }
}
