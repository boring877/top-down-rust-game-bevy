use crate::components::{Boss, Enemy, Health, Player, BladeAttack, BladeMaterial, BossAttackTimer, DamageNumber, BossMaterial, BossLastPosition};
use crate::constants::*;
use bevy::prelude::*;
use bevy::sprite_render::MeshMaterial2d;
use avian2d::prelude::*;

#[allow(dead_code)]
pub fn spawn_boss(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BossMaterial>>,
) {
    let material_handle = materials.add(BossMaterial {
        color: LinearRgba::new(0.8, 0.2, 0.3, 1.0), // Dark red demonic color
        hit_flash: 0.0,
        health_percent: 1.0,
        time: 0.0,
        is_moving: 0.0,
    });

    commands.spawn((
        Boss,
        Enemy,
        RigidBody::Dynamic,
        Collider::circle(BOSS_COLLIDER_RADIUS),
        LinearDamping(5.0),
        AngularDamping(10.0),
        Mesh2d(meshes.add(Rectangle::new(BOSS_SIZE.x * 3.0, BOSS_SIZE.y * 3.0))),
        MeshMaterial2d(material_handle),
        Transform::from_xyz(200.0, 200.0, 0.0),
        Health::new(BOSS_HEALTH),
        BossAttackTimer {
            timer: Timer::from_seconds(BOSS_ATTACK_COOLDOWN, TimerMode::Repeating),
        },
        BossLastPosition(Vec2::new(200.0, 200.0)),
    ));
}

pub fn boss_chase_player(
    _time: Res<Time>,
    player_query: Query<&Position, With<Player>>,
    mut boss_query: Query<(&Position, &mut LinearVelocity), With<Boss>>,
) {
    if let Ok(player_pos) = player_query.single() {
        for (boss_pos, mut velocity) in boss_query.iter_mut() {
            let player_pos_2d = Vec2::new(player_pos.x, player_pos.y);
            let boss_pos_2d = Vec2::new(boss_pos.x, boss_pos.y);

            let direction = player_pos_2d - boss_pos_2d;
            let distance = direction.length();

            // Set velocity directly for crisp movement toward player
            if distance > 0.0 {
                let normalized = direction.normalize();
                velocity.0 = normalized * BOSS_SPEED;
            } else {
                velocity.0 = Vec2::ZERO;
            }
        }
    }
}

pub fn cleanup_boss(mut commands: Commands, bosses: Query<Entity, With<Boss>>) {
    for entity in bosses.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn enemy_death(
    mut commands: Commands,
    enemies: Query<(Entity, &Health), With<Enemy>>,
) {
    for (entity, health) in enemies.iter() {
        if health.is_dead() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn boss_attack(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BladeMaterial>>,
    player_query: Query<&Position, With<Player>>,
    mut boss_query: Query<(&Position, &mut BossAttackTimer), With<Boss>>,
) {
    // Get player position
    let player_pos = player_query.single().ok().map(|p| Vec2::new(p.x, p.y));

    for (boss_pos, mut attack_timer) in boss_query.iter_mut() {
        attack_timer.timer.tick(time.delta());

        if attack_timer.timer.just_finished() {
            let boss_pos_2d = Vec2::new(boss_pos.x, boss_pos.y);

            // Calculate direction towards player
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
                    color: LinearRgba::new(0.9, 0.2, 0.3, 1.0), // Bright red blade
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

        // Update progress (0 to 1)
        let progress = blade.timer.fraction();

        // Move blade towards direction
        transform.translation.x += blade.direction.x * blade.speed * time.delta_secs();
        transform.translation.y += blade.direction.y * blade.speed * time.delta_secs();

        // Get the actual material and update it
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.progress = progress;
        }

        // Despawn when done
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
            continue; // Already hit player, skip
        }

        for (player_transform, mut health, mut velocity) in player.iter_mut() {
            let distance = blade_transform
                .translation
                .distance(player_transform.translation);

            // Check if player is in blade range
            if distance < BOSS_BLADE_RANGE {
                health.take_damage(blade.damage);
                blade.has_hit_player = true;

                // Apply knockback to player
                let knockback_dir = -blade.direction; // Opposite of blade direction
                velocity.x += knockback_dir.x * 200.0;
                velocity.y += knockback_dir.y * 200.0;

                // Spawn damage number for player (red color to show player damage)
                spawn_player_damage_number(
                    &mut commands,
                    blade.damage,
                    player_transform.translation.xy(),
                );

                break; // Only hit once per blade
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
        TextColor(Color::srgb(1.0, 0.2, 0.2)), // Red color for player damage
        Transform::from_xyz(position.x, position.y + DAMAGE_NUMBER_OFFSET_Y, 3.0),
    ));
}

// Animation system - updates boss shader uniforms
pub fn animate_boss(
    time: Res<Time>,
    mut materials: ResMut<Assets<BossMaterial>>,
    bosses: Query<(&Health, &Position, &LinearVelocity, &MeshMaterial2d<BossMaterial>), With<Boss>>,
) {
    for (health, _pos, velocity, material_handle) in bosses.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            // Update time for shader animation
            material.time += time.delta_secs();
            
            // Update health percent for rage effect
            material.health_percent = health.current as f32 / health.max as f32;
            
            // Check if moving for walk animation
            let speed = velocity.length();
            material.is_moving = if speed > 10.0 { 1.0 } else { 0.0 };
        }
    }
}

// Clamp boss position to arena boundaries
pub fn clamp_boss_to_arena(
    mut bosses: Query<&mut Position, With<Boss>>,
) {
    for mut pos in bosses.iter_mut() {
        pos.x = pos.x.clamp(-ARENA_HALF_WIDTH, ARENA_HALF_WIDTH);
        pos.y = pos.y.clamp(-ARENA_HALF_HEIGHT, ARENA_HALF_HEIGHT);
    }
}

// Test mode: Spawn boss resource
#[derive(Resource)]
pub struct BossSpawnTimer {
    pub timer: Timer,
}

impl Default for BossSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(BOSS_SPAWN_INTERVAL, TimerMode::Repeating),
        }
    }
}

// Test mode: Spawn bosses periodically
pub fn test_spawn_bosses(
    time: Res<Time>,
    mut timer: ResMut<BossSpawnTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BossMaterial>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if !TEST_MODE {
        return;
    }

    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        // Get player position to spawn away from player
        let player_pos = player_query.single().map(|t| t.translation.xy()).unwrap_or(Vec2::ZERO);

        // Random spawn position (away from player)
        let angle = rand_angle();
        let distance = 400.0 + rand_distance() * 400.0; // 400-800 units away
        let spawn_x = player_pos.x + angle.cos() * distance;
        let spawn_y = player_pos.y + angle.sin() * distance;

        // Clamp to arena
        let spawn_x = spawn_x.clamp(-ARENA_HALF_WIDTH + 100.0, ARENA_HALF_WIDTH - 100.0);
        let spawn_y = spawn_y.clamp(-ARENA_HALF_HEIGHT + 100.0, ARENA_HALF_HEIGHT - 100.0);

        let material_handle = materials.add(BossMaterial {
            color: LinearRgba::new(0.8, 0.2, 0.3, 1.0),
            hit_flash: 0.0,
            health_percent: 1.0,
            time: 0.0,
            is_moving: 0.0,
        });

        commands.spawn((
            Boss,
            Enemy,
            RigidBody::Dynamic,
            Collider::circle(BOSS_COLLIDER_RADIUS),
            LinearDamping(0.0), // No damping - direct velocity control
            AngularDamping(10.0),
            Mesh2d(meshes.add(Rectangle::new(BOSS_SIZE.x * 3.0, BOSS_SIZE.y * 3.0))),
            MeshMaterial2d(material_handle),
            Transform::from_xyz(spawn_x, spawn_y, 0.0),
            Health::new(BOSS_HEALTH),
            BossAttackTimer {
                timer: Timer::from_seconds(BOSS_ATTACK_COOLDOWN, TimerMode::Repeating),
            },
            BossLastPosition(Vec2::new(spawn_x, spawn_y)),
        ));
    }
}

// Simple random helpers (avoid pulling in rand crate)
fn rand_angle() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f32 / u32::MAX as f32) * std::f32::consts::TAU
}

fn rand_distance() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    nanos.wrapping_mul(2654435761) as f32 / u32::MAX as f32
}