use crate::components::{GameCamera, Player};
use bevy::prelude::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        GameCamera,
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 100.0),
    ));
}

pub fn update_camera(
    players: Query<&Transform, (With<Player>, Without<GameCamera>)>,
    mut cameras: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    for player_transform in players.iter() {
        for mut camera_transform in cameras.iter_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
            // Keep z at 1000 so camera sees everything below
        }
    }
}

pub fn cleanup_camera(mut commands: Commands, cameras: Query<Entity, With<GameCamera>>) {
    for entity in cameras.iter() {
        commands.entity(entity).despawn();
    }
}
