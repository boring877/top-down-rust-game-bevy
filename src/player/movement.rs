use crate::components::{Player, PlayerAction};
use crate::constants::*;
use bevy::prelude::*;
use avian2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn player_movement(
    _time: Res<Time>,
    mut players: Query<(&ActionState<PlayerAction>, &mut LinearVelocity), With<Player>>,
) {
    for (action_state, mut velocity) in players.iter_mut() {
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
            velocity.0 = direction * PLAYER_SPEED;
        } else {
            velocity.0 = Vec2::ZERO;
        }
    }
}

pub fn clamp_player_to_arena(
    mut players: Query<&mut Position, With<Player>>,
) {
    for mut pos in players.iter_mut() {
        pos.x = pos.x.clamp(-ARENA_HALF_WIDTH, ARENA_HALF_WIDTH);
        pos.y = pos.y.clamp(-ARENA_HALF_HEIGHT, ARENA_HALF_HEIGHT);
    }
}
