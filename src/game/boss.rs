use crate::components::{Boss, Enemy, Collider, Health, Player};
use crate::constants::*;
use bevy::prelude::*;

pub fn spawn_boss(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Boss,
        Enemy,
        Collider { radius: BOSS_COLLIDER_RADIUS },
        Sprite::from_image(asset_server.load("sprites/boss.png")),
        Transform::from_xyz(200.0, 200.0, 0.0),
        Health::new(BOSS_HEALTH),
    ));
}

pub fn boss_chase_player(
    player_query: Query<&Transform, With<Player>>,
    mut boss_query: Query<&mut Transform, (With<Boss>, Without<Player>)>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.single() {
        for mut boss_transform in boss_query.iter_mut() {
            let direction = Vec2::new(
                player_transform.translation.x - boss_transform.translation.x,
                player_transform.translation.y - boss_transform.translation.y,
            );

            if direction.length() > 0.0 {
                let normalized = direction.normalize();
                boss_transform.translation.x += normalized.x * BOSS_SPEED * time.delta_secs();
                boss_transform.translation.y += normalized.y * BOSS_SPEED * time.delta_secs();
            }
        }
    }
}

pub fn cleanup_boss(mut commands: Commands, bosses: Query<Entity, With<Boss>>) {
    for entity in bosses.iter() {
        commands.entity(entity).despawn();
    }
}
