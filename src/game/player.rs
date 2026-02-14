use crate::components::{Player, PlayerAction, default_input_map};
use crate::constants::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Sprite::from_image(asset_server.load("sprites/player.png")),
        Transform::from_xyz(0.0, 0.0, 1.0),
        default_input_map(),
    ));
}

pub fn player_movement(
    time: Res<Time>,
    mut players: Query<(&ActionState<PlayerAction>, &mut Transform), With<Player>>,
) {
    for (action_state, mut transform) in players.iter_mut() {
        let mut direction = Vec2::ZERO;

        if action_state.pressed(&PlayerAction::MoveUp) {
            direction.y += 1.0;
        }
        if action_state.pressed(&PlayerAction::MoveDown) {
            direction.y -= 1.0;
        }
        if action_state.pressed(&PlayerAction::MoveLeft) {
            direction.x -= 1.0;
        }
        if action_state.pressed(&PlayerAction::MoveRight) {
            direction.x += 1.0;
        }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
            transform.translation.x += direction.x * PLAYER_SPEED * time.delta_secs();
            transform.translation.y += direction.y * PLAYER_SPEED * time.delta_secs();
        }
    }
}

pub fn cleanup_player(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for entity in players.iter() {
        commands.entity(entity).despawn();
    }
}
