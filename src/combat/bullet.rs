use crate::components::{Bullet, Player, Enemy, FireRate, ShotCounter, SuperBullet, Obstacle, Health};
use crate::materials::BulletMaterial;
use crate::constants::*;
use bevy::prelude::*;
use bevy::sprite_render::MeshMaterial2d;
use avian2d::prelude::*;
use super::damage::spawn_damage_number;
use std::f32::consts::FRAC_PI_2;

pub fn spawn_bullet(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BulletMaterial>>,
    mut player_query: Query<(&Transform, &mut FireRate, &mut ShotCounter), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for (transform, mut fire_rate, mut shot_counter) in player_query.iter_mut() {
        fire_rate.timer.tick(time.delta());

        if fire_rate.timer.just_finished() {
            shot_counter.count += 1;

            // Find nearest enemy for bullet direction
            let (direction, angle) = find_nearest_enemy_direction_and_angle(
                transform.translation.xy(),
                &enemy_query
            );

            let is_super = shot_counter.count % SUPER_BULLET_INTERVAL == 0;

            let bullet_rotation = Quat::from_rotation_z(angle);

            if is_super {
                commands.spawn((
                    Bullet {
                        speed: SUPER_BULLET_SPEED,
                        damage: SUPER_BULLET_DAMAGE,
                        direction
                    },
                    SuperBullet,
                    Mesh2d(meshes.add(Rectangle::new(48.0, 48.0))),
                    MeshMaterial2d(materials.add(BulletMaterial {
                        color: LinearRgba::new(0.2, 0.8, 1.0, 1.0),
                    })),
                    Transform::from_xyz(transform.translation.x, transform.translation.y, 2.0)
                        .with_rotation(bullet_rotation),
                ));
            } else {
                commands.spawn((
                    Bullet {
                        speed: BULLET_SPEED,
                        damage: BULLET_DAMAGE,
                        direction
                    },
                    Mesh2d(meshes.add(Rectangle::new(20.0, 12.0))),
                    MeshMaterial2d(materials.add(BulletMaterial {
                        color: LinearRgba::new(1.0, 0.5, 0.1, 1.0),
                    })),
                    Transform::from_xyz(transform.translation.x, transform.translation.y, 2.0)
                        .with_rotation(bullet_rotation),
                ));
            }
        }
    }
}

/// Find direction and angle to nearest enemy
fn find_nearest_enemy_direction_and_angle(
    player_pos: Vec2,
    enemy_query: &Query<&Transform, With<Enemy>>,
) -> (Vec2, f32) {
    let mut nearest_distance = f32::MAX;
    let mut nearest_direction = Vec2::X; // Default direction (right)
    let mut nearest_angle = 0.0;

    for enemy_transform in enemy_query.iter() {
        let enemy_pos = enemy_transform.translation.xy();
        let distance = player_pos.distance(enemy_pos);

        if distance < nearest_distance {
            nearest_distance = distance;
            let diff = enemy_pos - player_pos;
            if diff != Vec2::ZERO {
                nearest_direction = diff.normalize();
                nearest_angle = diff.to_angle() - FRAC_PI_2;
            }
        }
    }

    (nearest_direction, nearest_angle)
}

pub fn bullet_movement_and_collision(
    time: Res<Time>,
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform, &Bullet), Without<Enemy>>,
    mut enemies: Query<(&Transform, &mut Health, &mut LinearVelocity), (With<Enemy>, Without<Obstacle>, Without<Bullet>)>,
    obstacles: Query<&Transform, (With<Obstacle>, Without<Enemy>, Without<Bullet>)>,
) {
    'bullet_loop: for (entity, mut transform, bullet) in bullets.iter_mut() {
        // Move bullet
        transform.translation.x += bullet.direction.x * bullet.speed * time.delta_secs();
        transform.translation.y += bullet.direction.y * bullet.speed * time.delta_secs();

        // Check if out of bounds
        let margin = 100.0;
        if transform.translation.x.abs() > ARENA_HALF_WIDTH + margin
            || transform.translation.y.abs() > ARENA_HALF_HEIGHT + margin {
            commands.entity(entity).despawn();
            continue 'bullet_loop;
        }

        // Check obstacle collision
        for obstacle_transform in obstacles.iter() {
            let distance = transform.translation.distance(obstacle_transform.translation);
            if distance < OBSTACLE_SIZE / 2.0 {
                commands.entity(entity).despawn();
                continue 'bullet_loop;
            }
        }

        // Check enemy collision
        for (enemy_transform, mut health, mut velocity) in enemies.iter_mut() {
            let distance = transform.translation.distance(enemy_transform.translation);

            if distance < BOSS_COLLIDER_RADIUS {
                health.take_damage(bullet.damage);

                let knockback_strength = if bullet.damage > BULLET_DAMAGE {
                    300.0
                } else {
                    150.0
                };
                velocity.x += bullet.direction.x * knockback_strength;
                velocity.y += bullet.direction.y * knockback_strength;

                spawn_damage_number(
                    &mut commands,
                    bullet.damage,
                    enemy_transform.translation.xy(),
                );

                commands.entity(entity).despawn();
                continue 'bullet_loop;
            }
        }
    }
}

pub fn cleanup_bullets(mut commands: Commands, bullets: Query<Entity, With<Bullet>>) {
    for entity in bullets.iter() {
        commands.entity(entity).despawn();
    }
}
