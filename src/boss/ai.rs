use crate::components::{Boss, Player, Health};
use crate::materials::BossMaterial;
use crate::constants::*;
use bevy::prelude::*;
use bevy::sprite_render::MeshMaterial2d;
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
    mut materials: ResMut<Assets<BossMaterial>>,
    bosses: Query<(&Health, &Position, &LinearVelocity, &MeshMaterial2d<BossMaterial>), With<Boss>>,
) {
    for (health, _pos, velocity, material_handle) in bosses.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.time += time.delta_secs();
            material.health_percent = health.current as f32 / health.max as f32;
            let speed = velocity.length();
            material.is_moving = if speed > 10.0 { 1.0 } else { 0.0 };
        }
    }
}
