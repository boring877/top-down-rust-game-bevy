use crate::components::Floor;
use crate::materials::FloorMaterial;
use crate::constants::*;
use bevy::prelude::*;
use bevy::sprite_render::MeshMaterial2d;

pub fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FloorMaterial>>,
    existing_floors: Query<(), With<Floor>>,
) {
    // Don't spawn if floor already exists (e.g., returning from pause)
    if !existing_floors.is_empty() {
        return;
    }

    let floor_width = ARENA_HALF_WIDTH * 4.0;
    let floor_height = ARENA_HALF_HEIGHT * 4.0;

    commands.spawn((
        Floor,
        Mesh2d(meshes.add(Rectangle::new(floor_width, floor_height))),
        MeshMaterial2d(materials.add(FloorMaterial {
            color: FLOOR_COLOR.to_linear(),
            tile_size: 64.0,
            _pad1: 0.0,
            _pad2: 0.0,
        })),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
}

pub fn cleanup_floor(mut commands: Commands, floors: Query<Entity, With<Floor>>) {
    for entity in floors.iter() {
        commands.entity(entity).despawn();
    }
}
