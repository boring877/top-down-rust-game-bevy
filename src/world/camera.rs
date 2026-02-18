use crate::components::{GameCamera, Player};
use crate::constants::*;
use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands, existing_cameras: Query<(), With<GameCamera>>) {
    // Don't spawn if camera already exists (e.g., returning from pause)
    if !existing_cameras.is_empty() {
        return;
    }

    commands.spawn((
        GameCamera,
        Camera2d,
        Camera {
            order: 1,  // Higher order than default (0) to avoid ambiguity warnings
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 100.0),
    ));
}

pub fn update_camera(
    players: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    mut cameras: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    let camera_margin_x = 640.0;
    let camera_margin_y = 400.0;

    let max_cam_x = ARENA_HALF_WIDTH - camera_margin_x;
    let max_cam_y = ARENA_HALF_HEIGHT - camera_margin_y;

    for player_transform in players.iter() {
        for mut camera_transform in cameras.iter_mut() {
            let target_x = player_transform.translation.x.clamp(-max_cam_x, max_cam_x);
            let target_y = player_transform.translation.y.clamp(-max_cam_y, max_cam_y);

            camera_transform.translation.x = target_x;
            camera_transform.translation.y = target_y;
        }
    }
}

pub fn cleanup_camera(mut commands: Commands, cameras: Query<Entity, With<GameCamera>>) {
    for entity in cameras.iter() {
        commands.entity(entity).despawn();
    }
}
