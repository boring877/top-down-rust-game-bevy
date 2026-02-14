use crate::components::Floor;
use crate::constants::*;
use bevy::prelude::*;

pub fn spawn_floor(mut commands: Commands) {
    for x in -GRID_SIZE / 2..GRID_SIZE / 2 {
        for y in -GRID_SIZE / 2..GRID_SIZE / 2 {
            let is_checker = (x + y) % 2 == 0;
            let color = if is_checker { FLOOR_COLOR } else { GRID_COLOR };

            let pos_x = (x as f32 + 0.5) * TILE_SIZE;
            let pos_y = (y as f32 + 0.5) * TILE_SIZE;

            commands.spawn((
                Floor,
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(pos_x, pos_y, -10.0),
            ));
        }
    }
}

pub fn cleanup_floor(mut commands: Commands, floors: Query<Entity, With<Floor>>) {
    for entity in floors.iter() {
        commands.entity(entity).despawn();
    }
}
