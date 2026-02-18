use crate::components::{Boss, Player, BladeAttack, BossAttackTimer, DamageNumber, Health};
use crate::materials::BladeMaterial;
use crate::constants::*;
use bevy::prelude::*;
use bevy::sprite_render::MeshMaterial2d;
use avian2d::prelude::*;

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
    mut player: Query<(&Transform, &mut Health, &mut LinearVelocity), With<Player>>,
) {
    for (mut blade, blade_transform) in blades.iter_mut() {
        if blade.has_hit_player {
            continue;
        }

        for (player_transform, mut health, mut velocity) in player.iter_mut() {
            let distance = blade_transform
                .translation
                .distance(player_transform.translation);

            if distance < BOSS_BLADE_RANGE {
                health.take_damage(blade.damage);
                blade.has_hit_player = true;

                let knockback_dir = -blade.direction;
                velocity.x += knockback_dir.x * 200.0;
                velocity.y += knockback_dir.y * 200.0;

                spawn_player_damage_number(
                    &mut commands,
                    blade.damage,
                    player_transform.translation.xy(),
                );

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
