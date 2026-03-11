use crate::components::{Boss, Player, Health};
use crate::constants::*;
use bevy::prelude::*;
use avian2d::prelude::*;

pub fn boss_chase_player(
    _time: Res<Time>,
    player_query: Query<&Position, With<Player>>,
    mut boss_query: Query<(&Position, &mut LinearVelocity), With<Boss>>,
) {
    if let Ok(player_pos) = player_query.single() {
        for (boss_pos, mut velocity) in boss_query.iter_mut() {
            let player_pos_2d = Vec2::new(player_pos.x, player_pos.y);
            let boss_pos_2d = Vec2::new(boss_pos.x, boss_pos.y);

            let direction = player_pos_2d - boss_pos_2d;
            let distance = direction.length();

            if distance > 0.0 {
                let normalized = direction.normalize();
                velocity.0 = normalized * BOSS_SPEED;
            } else {
                velocity.0 = Vec2::ZERO;
            }
        }
    }
}

pub fn clamp_boss_to_arena(
    mut bosses: Query<&mut Position, With<Boss>>,
) {
    for mut pos in bosses.iter_mut() {
        pos.x = pos.x.clamp(-ARENA_HALF_WIDTH, ARENA_HALF_WIDTH);
        pos.y = pos.y.clamp(-ARENA_HALF_HEIGHT, ARENA_HALF_HEIGHT);
    }
}

pub fn animate_boss(
    time: Res<Time>,
    mut bosses: Query<(&LinearVelocity, &mut Sprite, &mut Transform), With<Boss>>,
) {
    let t = time.elapsed_secs() * 5.0; // Slower bobbing for big boss

    for (velocity, mut sprite, mut transform) in bosses.iter_mut() {
        let speed = velocity.length();

        if velocity.x < -0.1 {
            sprite.flip_x = true;
        } else if velocity.x > 0.1 {
            sprite.flip_x = false;
        }

        if speed > 10.0 {
            transform.scale.x = 1.0 + (t.sin() * 0.05);
            transform.scale.y = 1.0 + (t.cos() * 0.05);
        } else {
            transform.scale.x = 1.0;
            transform.scale.y = 1.0;
        }
    }
}
