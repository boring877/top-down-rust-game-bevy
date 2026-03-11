use crate::components::{Boss, Player, BladeAttack, BossAttackTimer, DamageNumber, Health};
use crate::materials::BladeMaterial;
use crate::constants::*;
use bevy::prelude::*;
use bevy::sprite_render::MeshMaterial2d;
use avian2d::prelude::*;
use rand::RngExt;

pub fn boss_attack(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BladeMaterial>>,
    player_query: Query<&Position, With<Player>>,
    mut boss_query: Query<(&Position, &mut BossAttackTimer), With<Boss>>,
) {
    let player_pos = player_query.single().ok().map(|p| Vec2::new(p.x, p.y));

    for (boss_pos, mut attack_timer) in boss_query.iter_mut() {
        attack_timer.timer.tick(time.delta());

        if attack_timer.timer.just_finished() {
            let boss_pos_2d = Vec2::new(boss_pos.x, boss_pos.y);

            let direction = if let Some(player) = player_pos {
                let diff = player - boss_pos_2d;
                if diff.length() > 0.0 {
                    diff.normalize()
                } else {
                    Vec2::X
                }
            } else {
                Vec2::X
            };

            let attack_type = rand::rng().random_range(0..3);
            
            match attack_type {
                0 => {
                    // Standard Blade
                    commands.spawn((
                        BladeAttack {
                            timer: Timer::from_seconds(BOSS_BLADE_DURATION, TimerMode::Once),
                            damage: BOSS_BLADE_DAMAGE,
                            angle_start: 0.0,
                            angle_end: 3.14159 * 2.0,
                            has_hit_player: false,
                            direction,
                            speed: BOSS_BLADE_SPEED,
                        },
                        Mesh2d(meshes.add(Rectangle::new(BOSS_BLADE_SIZE, BOSS_BLADE_SIZE))),
                        MeshMaterial2d(materials.add(BladeMaterial {
                            color: LinearRgba::new(0.9, 0.2, 0.3, 1.0),
                            progress: 0.0,
                        })),
                        Transform::from_xyz(boss_pos_2d.x, boss_pos_2d.y, 2.5)
                            .with_rotation(Quat::from_rotation_z(direction.y.atan2(direction.x))),
                    ));
                }
                1 => {
                    // Triple Spread Blades
                    for i in -1..=1 {
                        let offset_dir = Vec2::new(
                            direction.x * (i as f32 * 0.3).cos() - direction.y * (i as f32 * 0.3).sin(),
                            direction.x * (i as f32 * 0.3).sin() + direction.y * (i as f32 * 0.3).cos()
                        ).normalize_or_zero();
                        
                        commands.spawn((
                            BladeAttack {
                                timer: Timer::from_seconds(BOSS_BLADE_DURATION, TimerMode::Once),
                                damage: BOSS_BLADE_DAMAGE / 2,
                                angle_start: 0.0,
                                angle_end: 3.14159 * 2.0,
                                has_hit_player: false,
                                direction: offset_dir,
                                speed: BOSS_BLADE_SPEED * 1.5,
                            },
                            Mesh2d(meshes.add(Rectangle::new(BOSS_BLADE_SIZE * 0.6, BOSS_BLADE_SIZE * 0.6))),
                            MeshMaterial2d(materials.add(BladeMaterial {
                                color: LinearRgba::new(1.0, 0.5, 0.0, 1.0), // Orange
                                progress: 0.0,
                            })),
                            Transform::from_xyz(boss_pos_2d.x, boss_pos_2d.y, 2.5)
                                .with_rotation(Quat::from_rotation_z(offset_dir.y.atan2(offset_dir.x))),
                        ));
                    }
                }
                _ => {
                    // Giant slow blade
                    commands.spawn((
                        BladeAttack {
                            timer: Timer::from_seconds(BOSS_BLADE_DURATION * 1.5, TimerMode::Once),
                            damage: BOSS_BLADE_DAMAGE * 2,
                            angle_start: 0.0,
                            angle_end: 3.14159 * 2.0,
                            has_hit_player: false,
                            direction,
                            speed: BOSS_BLADE_SPEED * 0.6,
                        },
                        Mesh2d(meshes.add(Rectangle::new(BOSS_BLADE_SIZE * 2.0, BOSS_BLADE_SIZE * 2.0))),
                        MeshMaterial2d(materials.add(BladeMaterial {
                            color: LinearRgba::new(0.6, 0.0, 0.8, 1.0), // Purple
                            progress: 0.0,
                        })),
                        Transform::from_xyz(boss_pos_2d.x, boss_pos_2d.y, 2.5)
                            .with_rotation(Quat::from_rotation_z(direction.y.atan2(direction.x))),
                    ));
                }
            }
        }
    }
}

pub fn animate_blade(
    time: Res<Time>,
    mut commands: Commands,
    mut materials: ResMut<Assets<BladeMaterial>>,
    mut blades: Query<(Entity, &mut BladeAttack, &mut Transform, &MeshMaterial2d<BladeMaterial>)>,
) {
    for (entity, mut blade, mut transform, material_handle) in blades.iter_mut() {
        blade.timer.tick(time.delta());
        let progress = blade.timer.fraction();

        transform.translation.x += blade.direction.x * blade.speed * time.delta_secs();
        transform.translation.y += blade.direction.y * blade.speed * time.delta_secs();

        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.progress = progress;
        }

        if blade.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn blade_collision(
    mut commands: Commands,
    mut blades: Query<(&mut BladeAttack, &Transform)>,
    mut player: Query<(&Transform, &mut Health, &mut LinearVelocity, &crate::components::CombatStats), With<Player>>,
    equipment: Res<crate::components::PlayerEquipment>,
) {
    let mut rng = rand::rng();

    for (mut blade, blade_transform) in blades.iter_mut() {
        if blade.has_hit_player {
            continue;
        }

        for (player_transform, mut health, mut velocity, base_stats) in player.iter_mut() {
            let distance = blade_transform
                .translation
                .distance(player_transform.translation);

            if distance < BOSS_BLADE_RANGE {
                blade.has_hit_player = true;
                let total_stats = equipment.get_total_stats(base_stats);

                if rng.random_range(0.0..1.0) < total_stats.dodge_rate {
                    spawn_dodge_text(&mut commands, player_transform.translation.xy());
                } else {
                    health.take_damage(blade.damage);

                    let knockback_dir = -blade.direction;
                    velocity.x += knockback_dir.x * 200.0;
                    velocity.y += knockback_dir.y * 200.0;

                    spawn_player_damage_number(
                        &mut commands,
                        blade.damage,
                        player_transform.translation.xy(),
                    );
                }

                break;
            }
        }
    }
}

fn spawn_player_damage_number(commands: &mut Commands, damage: u32, position: Vec2) {
    commands.spawn((
        DamageNumber {
            timer: Timer::from_seconds(DAMAGE_NUMBER_DURATION, TimerMode::Once),
            damage,
        },
        Text2d::new(format!("-{}", damage)),
        TextFont {
            font_size: DAMAGE_NUMBER_FONT_SIZE,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.2, 0.2)),
        Transform::from_xyz(position.x, position.y + DAMAGE_NUMBER_OFFSET_Y, 3.0),
    ));
}

fn spawn_dodge_text(commands: &mut Commands, position: Vec2) {
    commands.spawn((
        DamageNumber {
            timer: Timer::from_seconds(DAMAGE_NUMBER_DURATION, TimerMode::Once),
            damage: 0,
        },
        Text2d::new("DODGED!"),
        TextFont {
            font_size: DAMAGE_NUMBER_FONT_SIZE,
            ..default()
        },
        TextColor(Color::srgb(0.6, 0.8, 1.0)),
        Transform::from_xyz(position.x, position.y + DAMAGE_NUMBER_OFFSET_Y, 3.0),
    ));
}
