use crate::components::{Health, Player, FireRate, ShotCounter};
use crate::materials::PlayerMaterial;
use crate::constants::*;
use bevy::prelude::*;
use bevy::sprite_render::MeshMaterial2d;
use avian2d::prelude::*;
use crate::components::default_input_map;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlayerMaterial>>,
    existing_players: Query<(), With<Player>>,
) {
    // Don't spawn if player already exists (e.g., returning from pause)
    if !existing_players.is_empty() {
        return;
    }

    let material_handle = materials.add(PlayerMaterial {
        color: LinearRgba::new(0.2, 0.6, 0.9, 1.0),
        hit_flash: 0.0,
        facing_angle: 0.0,
        is_moving: 0.0,
        time: 0.0,
    });

    commands.spawn((
        FireRate::new(DEFAULT_FIRE_RATE),
        ShotCounter::default(),
        Player,
        RigidBody::Dynamic,
        Collider::circle(PLAYER_SIZE.x / 2.0),
        LinearDamping(5.0),
        AngularDamping(10.0),
        Mesh2d(meshes.add(Rectangle::new(PLAYER_SIZE.x * 6.0, PLAYER_SIZE.y * 6.0))),
        MeshMaterial2d(material_handle.clone()),
        Transform::from_xyz(0.0, 0.0, 1.0),
        default_input_map(),
        Health::new(PLAYER_HEALTH),
    ));
}

pub fn cleanup_player(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for entity in players.iter() {
        commands.entity(entity).despawn();
    }
}
