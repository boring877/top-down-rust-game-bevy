use crate::components::{Obstacle, ObstacleMaterial, ObstacleShape};
use crate::constants::*;
use bevy::prelude::*;
use bevy::sprite_render::MeshMaterial2d;
use avian2d::prelude::*;

pub fn spawn_obstacles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ObstacleMaterial>>,
) {
    for (x, y, shape_type) in ARENA_OBSTACLES {
        let shape = match shape_type {
            0 => ObstacleShape::Rock,
            1 => ObstacleShape::Crystal,
            _ => ObstacleShape::Pillar,
        };

        let color = match shape {
            ObstacleShape::Rock => OBSTACLE_ROCK_COLOR,
            ObstacleShape::Crystal => OBSTACLE_CRYSTAL_COLOR,
            ObstacleShape::Pillar => OBSTACLE_PILLAR_COLOR,
        };

        let linear_color = color.to_linear();

        commands.spawn((
            Obstacle,
            RigidBody::Static, // Static obstacles don't move
            Collider::circle(OBSTACLE_SIZE / 2.0),
            Mesh2d(meshes.add(Rectangle::new(OBSTACLE_SIZE, OBSTACLE_SIZE))),
            MeshMaterial2d(materials.add(ObstacleMaterial {
                color: LinearRgba::new(linear_color.red, linear_color.green, linear_color.blue, 1.0),
                shape_type: shape_type as f32,
                _pad1: 0.0,
                _pad2: 0.0,
            })),
            Transform::from_xyz(x, y, 1.0),
        ));
    }
}

pub fn cleanup_obstacles(mut commands: Commands, obstacles: Query<Entity, With<Obstacle>>) {
    for entity in obstacles.iter() {
        commands.entity(entity).despawn();
    }
}