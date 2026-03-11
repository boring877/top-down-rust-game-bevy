use crate::components::{Boss, Enemy, Health, Player, BossAttackTimer, BossLastPosition};
use crate::materials::BossMaterial;
use crate::constants::*;
use bevy::prelude::*;
use bevy::sprite_render::MeshMaterial2d;
use avian2d::prelude::*;
use super::BossSpawnTimer;

pub fn test_spawn_bosses(
    time: Res<Time>,
    mut timer: ResMut<BossSpawnTimer>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if !TEST_MODE {
        return;
    }

    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        let player_pos = player_query.single().map(|t| t.translation.xy()).unwrap_or(Vec2::ZERO);

        let angle = rand_angle();
        let distance = 400.0 + rand_distance() * 400.0;
        let spawn_x = player_pos.x + angle.cos() * distance;
        let spawn_y = player_pos.y + angle.sin() * distance;

        let spawn_x = spawn_x.clamp(-ARENA_HALF_WIDTH + 100.0, ARENA_HALF_WIDTH - 100.0);
        let spawn_y = spawn_y.clamp(-ARENA_HALF_HEIGHT + 100.0, ARENA_HALF_HEIGHT - 100.0);

        commands.spawn((
            Boss,
            Enemy,
            RigidBody::Kinematic,
            Collider::circle(BOSS_COLLIDER_RADIUS),
            Sensor,
            LinearDamping(0.0),
            AngularDamping(10.0),
            Sprite {
                image: asset_server.load("sprites/boss.png"),
                custom_size: Some(Vec2::new(BOSS_SIZE.x * 3.0, BOSS_SIZE.y * 3.0)),
                texture_atlas: Some(TextureAtlas {
                    layout: layouts.add(TextureAtlasLayout::from_grid(UVec2::new(192, 192), 6, 1, None, None)),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_xyz(spawn_x, spawn_y, 0.0),
            Health::new(BOSS_HEALTH),
            BossAttackTimer {
                timer: Timer::from_seconds(BOSS_ATTACK_COOLDOWN, TimerMode::Repeating),
            },
            BossLastPosition(Vec2::new(spawn_x, spawn_y)),
            crate::components::CombatStats {
                intelligence: 5,
                strength: 20,
                agility: 5,
                crit_rate: 0.05,
                crit_damage: 1.5,
                dodge_rate: 0.0,
            },
        ));
    }
}

pub fn cleanup_boss(mut commands: Commands, bosses: Query<Entity, With<Boss>>) {
    for entity in bosses.iter() {
        commands.entity(entity).despawn();
    }
}

/// Reset the boss spawn timer when entering the game
pub fn reset_boss_timer(mut timer: ResMut<BossSpawnTimer>) {
    timer.timer.reset();
}

fn rand_angle() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f32 / u32::MAX as f32) * std::f32::consts::TAU
}

fn rand_distance() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    nanos.wrapping_mul(2654435761) as f32 / u32::MAX as f32
}
