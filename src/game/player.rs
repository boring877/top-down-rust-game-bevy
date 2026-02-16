use crate::components::{Health, Player, PlayerAction, default_input_map, FireRate, ShotCounter, PlayerMaterial};
use crate::constants::*;
use bevy::prelude::*;
use bevy::sprite_render::MeshMaterial2d;
use avian2d::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlayerMaterial>>,
) {
    let material_handle = materials.add(PlayerMaterial {
        color: LinearRgba::new(0.2, 0.6, 0.9, 1.0), // Cyan sci-fi color
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
        LinearDamping(0.0), // No damping - direct velocity control
        AngularDamping(10.0), // Prevent rotation
        Mesh2d(meshes.add(Rectangle::new(PLAYER_SIZE.x * 6.0, PLAYER_SIZE.y * 6.0))),
        MeshMaterial2d(material_handle.clone()),
        Transform::from_xyz(0.0, 0.0, 1.0),
        default_input_map(),
        Health::new(PLAYER_HEALTH),
    ));
}

pub fn player_movement(
    _time: Res<Time>,
    mut players: Query<(&ActionState<PlayerAction>, &mut LinearVelocity), With<Player>>,
) {
    for (action_state, mut velocity) in players.iter_mut() {
        let mut direction = Vec2::ZERO;

        if action_state.pressed(&PlayerAction::MoveUp) {
            direction.y += 1.0;
        }
        if action_state.pressed(&PlayerAction::MoveDown) {
            direction.y -= 1.0;
        }
        if action_state.pressed(&PlayerAction::MoveLeft) {
            direction.x -= 1.0;
        }
        if action_state.pressed(&PlayerAction::MoveRight) {
            direction.x += 1.0;
        }

        // Set velocity directly for crisp, responsive movement
        if direction != Vec2::ZERO {
            direction = direction.normalize();
            velocity.0 = direction * PLAYER_SPEED;
        } else {
            velocity.0 = Vec2::ZERO;
        }
    }
}

pub fn cleanup_player(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for entity in players.iter() {
        commands.entity(entity).despawn();
    }
}

// Animation system - updates player shader uniforms
pub fn animate_player(
    time: Res<Time>,
    mut materials: ResMut<Assets<PlayerMaterial>>,
    players: Query<(&ActionState<PlayerAction>, &MeshMaterial2d<PlayerMaterial>), With<Player>>,
) {
    for (action_state, material_handle) in players.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            // Update time for shader animation
            material.time += time.delta_secs();
            
            // Check if moving
            let is_moving = action_state.pressed(&PlayerAction::MoveUp)
                || action_state.pressed(&PlayerAction::MoveDown)
                || action_state.pressed(&PlayerAction::MoveLeft)
                || action_state.pressed(&PlayerAction::MoveRight);
            
            material.is_moving = if is_moving { 1.0 } else { 0.0 };
        }
    }
}

// Clamp player position to arena boundaries
pub fn clamp_player_to_arena(
    mut players: Query<&mut Position, With<Player>>,
) {
    for mut pos in players.iter_mut() {
        pos.x = pos.x.clamp(-ARENA_HALF_WIDTH, ARENA_HALF_WIDTH);
        pos.y = pos.y.clamp(-ARENA_HALF_HEIGHT, ARENA_HALF_HEIGHT);
    }
}