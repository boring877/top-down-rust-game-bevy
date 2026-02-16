use crate::components::{Bullet, Health, Player, Enemy, FireRate, BulletMaterial, ShotCounter, SuperBullet, DamageNumber, Obstacle};
use crate::constants::*;
use bevy::prelude::*;
use bevy::sprite_render::MeshMaterial2d;
use avian2d::prelude::*;

pub fn spawn_bullet(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BulletMaterial>>,
    mut player_query: Query<(&Transform, &mut FireRate, &mut ShotCounter), With<Player>>,
) {
    for (transform, mut fire_rate, mut shot_counter) in player_query.iter_mut() {
        fire_rate.timer.tick(time.delta());

        if fire_rate.timer.just_finished() {
            shot_counter.count += 1;
            let direction = Vec2::X;

            // Check if this should be a super bullet (every 4th shot)
            let is_super = shot_counter.count % SUPER_BULLET_INTERVAL == 0;

            if is_super {
                // Super bullet - big, slow, massive damage, cyan color
                commands.spawn((
                    Bullet {
                        speed: SUPER_BULLET_SPEED,
                        damage: SUPER_BULLET_DAMAGE,
                        direction
                    },
                    SuperBullet,
                    Mesh2d(meshes.add(Rectangle::new(48.0, 48.0))),
                    MeshMaterial2d(materials.add(BulletMaterial {
                        color: LinearRgba::new(0.2, 0.8, 1.0, 1.0), // Cyan
                    })),
                    Transform::from_xyz(transform.translation.x, transform.translation.y, 2.0),
                ));
            } else {
                // Normal bullet - orange
                commands.spawn((
                    Bullet {
                        speed: BULLET_SPEED,
                        damage: BULLET_DAMAGE,
                        direction
                    },
                    Mesh2d(meshes.add(Rectangle::new(20.0, 12.0))),
                    MeshMaterial2d(materials.add(BulletMaterial {
                        color: LinearRgba::new(1.0, 0.5, 0.1, 1.0), // Orange
                    })),
                    Transform::from_xyz(transform.translation.x, transform.translation.y, 2.0),
                ));
            }
        }
    }
}

pub fn bullet_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform, &Bullet)>,
) {
    for (entity, mut transform, bullet) in bullets.iter_mut() {
        transform.translation.x += bullet.direction.x * bullet.speed * time.delta_secs();
        transform.translation.y += bullet.direction.y * bullet.speed * time.delta_secs();

        // Despawn bullets that go outside arena bounds (with some margin)
        let margin = 100.0;
        if transform.translation.x.abs() > ARENA_HALF_WIDTH + margin
            || transform.translation.y.abs() > ARENA_HALF_HEIGHT + margin {
            commands.entity(entity).despawn();
        }
    }
}

pub fn bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &Transform, &Bullet), With<Bullet>>,
    mut enemies: Query<(&Transform, &mut Health, &mut LinearVelocity), (With<Enemy>, Without<Obstacle>)>,
    obstacles: Query<&Transform, (With<Obstacle>, Without<Enemy>)>,
) {
    'bullet_loop: for (bullet_entity, bullet_transform, bullet) in bullets.iter() {
        // Check collision with obstacles first
        for obstacle_transform in obstacles.iter() {
            let distance = bullet_transform
                .translation
                .distance(obstacle_transform.translation);

            // Use OBSTACLE_SIZE / 2 as collision radius
            if distance < OBSTACLE_SIZE / 2.0 {
                commands.entity(bullet_entity).despawn();
                continue 'bullet_loop;
            }
        }

        // Check collision with enemies
        for (enemy_transform, mut health, mut velocity) in enemies.iter_mut() {
            let distance = bullet_transform
            .translation
            .distance(enemy_transform.translation);

            // Use BOSS_COLLIDER_RADIUS for boss collision
            if distance < BOSS_COLLIDER_RADIUS {
                // Apply damage
                health.take_damage(bullet.damage);

                // Apply knockback to enemy
                let knockback_strength = if bullet.damage > BULLET_DAMAGE {
                    300.0 // Super bullet knockback
                } else {
                    150.0 // Normal bullet knockback
                };
                velocity.x += bullet.direction.x * knockback_strength;
                velocity.y += bullet.direction.y * knockback_strength;

                // Spawn damage number
                spawn_damage_number(
                    &mut commands,
                    bullet.damage,
                    enemy_transform.translation.xy(),
                );

                commands.entity(bullet_entity).despawn();
                break;
            }
        }
    }
}

fn spawn_damage_number(commands: &mut Commands, damage: u32, position: Vec2) {
    commands.spawn((
        DamageNumber {
            timer: Timer::from_seconds(DAMAGE_NUMBER_DURATION, TimerMode::Once),
            damage,
        },
        Text2d::new(format!("{}", damage)),
        TextFont {
            font_size: DAMAGE_NUMBER_FONT_SIZE,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.8, 0.2)), // Yellow/gold color
        Transform::from_xyz(position.x, position.y + DAMAGE_NUMBER_OFFSET_Y, 3.0),
    ));
}

pub fn animate_damage_numbers(
    time: Res<Time>,
    mut commands: Commands,
    mut damage_numbers: Query<(Entity, &mut DamageNumber, &mut Transform, &mut TextColor)>,
) {
    for (entity, mut damage_number, mut transform, mut color) in damage_numbers.iter_mut() {
        damage_number.timer.tick(time.delta());

        // Float upward
        transform.translation.y += DAMAGE_NUMBER_SPEED * time.delta_secs();

        // Fade out
        let progress = damage_number.timer.fraction();
        let alpha = 1.0 - progress;
        color.0 = color.0.with_alpha(alpha);

        // Despawn when timer finishes
        if damage_number.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn cleanup_bullets(mut commands: Commands, bullets: Query<Entity, With<Bullet>>) {
    for entity in bullets.iter() {
        commands.entity(entity).despawn();
    }
}