use crate::components::{Player, PlayerAction};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn animate_player(
    time: Res<Time>,
    mut players: Query<(&ActionState<PlayerAction>, &mut Sprite, &mut Transform), With<Player>>,
) {
    let t = time.elapsed_secs() * 10.0;
    
    for (action_state, mut sprite, mut transform) in players.iter_mut() {
        let is_moving = action_state.pressed(&PlayerAction::MoveUp)
            || action_state.pressed(&PlayerAction::MoveDown)
            || action_state.pressed(&PlayerAction::MoveLeft)
            || action_state.pressed(&PlayerAction::MoveRight);

        if action_state.pressed(&PlayerAction::MoveLeft) {
            sprite.flip_x = true;
        } else if action_state.pressed(&PlayerAction::MoveRight) {
            sprite.flip_x = false;
        }

        if is_moving {
            // Squash and stretch
            transform.scale.x = 1.0 + (t.sin() * 0.05);
            transform.scale.y = 1.0 + (t.cos() * 0.05);
        } else {
            transform.scale.x = 1.0;
            transform.scale.y = 1.0;
        }
    }
}
