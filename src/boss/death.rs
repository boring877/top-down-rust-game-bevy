use crate::components::{Enemy, Health, Boss};
use super::particles::{spawn_death_explosion, spawn_electric_effect, ParticleAssets};
use super::{Dying, DeathStage};
use bevy::prelude::*;

/// When boss health reaches 0, start the dying animation instead of instant despawn
pub fn enemy_death(
    mut commands: Commands,
    assets: Res<ParticleAssets>,
    enemies: Query<(Entity, &Health, &Transform), (With<Enemy>, Without<Dying>)>,
) {
    for (entity, health, transform) in enemies.iter() {
        if health.is_dead() {
            // Start dying animation - collapsing phase
            let electric_entity = spawn_electric_effect(
                &mut commands,
                &assets,
                transform.translation,
            );

            commands.entity(entity).insert(Dying {
                timer: Timer::from_seconds(1.5, TimerMode::Once), // 1.5s collapse time
                stage: DeathStage::Collapsing,
                original_scale: transform.scale,
                electric_entity: Some(electric_entity),
            });
        }
    }
}

/// Animate the dying boss - shrinking, electricity, then explosion
pub fn animate_boss_death(
    mut commands: Commands,
    assets: Res<ParticleAssets>,
    time: Res<Time>,
    mut dying_bosses: Query<(Entity, &mut Dying, &mut Transform), With<Boss>>,
) {
    use rand::RngExt;
    let mut rng = rand::rng();

    for (entity, mut dying, mut transform) in dying_bosses.iter_mut() {
        dying.timer.tick(time.delta());

        match dying.stage {
            DeathStage::Collapsing => {
                // Shrink the boss during collapse
                let progress = dying.timer.fraction();
                let shrink_factor = 1.0 - (progress * 0.7); // Shrink to 30% of original size
                transform.scale = dying.original_scale * shrink_factor;

                // Shake effect
                let shake = (time.elapsed_secs() * 30.0).sin() * 5.0 * (1.0 - progress);
                transform.translation.x += shake * time.delta_secs();

                // When collapse timer finishes, switch to exploding
                if dying.timer.just_finished() {
                    dying.stage = DeathStage::Exploding;
                    dying.timer = Timer::from_seconds(0.3, TimerMode::Once); // Short pause before explosion

                    // Remove electricity effect
                    if let Some(electric_entity) = dying.electric_entity {
                        commands.entity(electric_entity).despawn();
                    }
                }
            }
            DeathStage::Exploding => {
                // Final explosion after short delay
                if dying.timer.just_finished() {
                    let pos = transform.translation.xy();
                    
                    // Spawn Pickups
                    // 10 XP gems
                    for _ in 0..10 {
                        let offset = Vec2::new(rng.random_range(-50.0..50.0), rng.random_range(-50.0..50.0));
                        commands.spawn((
                            crate::components::Pickup {
                                pickup_type: crate::components::PickupType::Xp,
                                amount: 10,
                            },
                            Sprite {
                                color: Color::srgb(0.2, 0.8, 0.2),
                                custom_size: Some(Vec2::new(10.0, 10.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos.x + offset.x, pos.y + offset.y, 1.5),
                        ));
                    }
                    // 5 Gold
                    for _ in 0..5 {
                        let offset = Vec2::new(rng.random_range(-50.0..50.0), rng.random_range(-50.0..50.0));
                        commands.spawn((
                            crate::components::Pickup {
                                pickup_type: crate::components::PickupType::Gold,
                                amount: 5,
                            },
                            Sprite {
                                color: Color::srgb(1.0, 0.8, 0.0),
                                custom_size: Some(Vec2::new(8.0, 8.0)),
                                ..default()
                            },
                            Transform::from_xyz(pos.x + offset.x, pos.y + offset.y, 1.5),
                        ));
                    }
                    // 1 Material
                    let offset = Vec2::new(rng.random_range(-50.0..50.0), rng.random_range(-50.0..50.0));
                    commands.spawn((
                        crate::components::Pickup {
                            pickup_type: crate::components::PickupType::Material,
                            amount: 1,
                        },
                        Sprite {
                            color: Color::srgb(0.8, 0.2, 0.8),
                            custom_size: Some(Vec2::new(12.0, 12.0)),
                            ..default()
                        },
                        Transform::from_xyz(pos.x + offset.x, pos.y + offset.y, 1.5),
                    ));

                    // 1 Equipment item
                    let equip_types = [
                        crate::components::PickupType::Weapon,
                        crate::components::PickupType::Helmet,
                        crate::components::PickupType::Armor,
                        crate::components::PickupType::Pants,
                        crate::components::PickupType::Shoes,
                        crate::components::PickupType::Ring,
                        crate::components::PickupType::Earring,
                        crate::components::PickupType::Necklace,
                        crate::components::PickupType::Gemstone,
                    ];
                    let random_equip = equip_types[rng.random_range(0..equip_types.len())];
                    
                    let offset = Vec2::new(rng.random_range(-50.0..50.0), rng.random_range(-50.0..50.0));
                    commands.spawn((
                        crate::components::Pickup {
                            pickup_type: random_equip,
                            amount: 1, // Placeholder
                        },
                        Sprite {
                            color: Color::srgb(0.5, 1.0, 0.5), // Distinct color for equipment
                            custom_size: Some(Vec2::new(15.0, 15.0)),
                            ..default()
                        },
                        Transform::from_xyz(pos.x + offset.x, pos.y + offset.y, 1.5),
                    ));

                    // 1 Skill Gem
                    let offset = Vec2::new(rng.random_range(-50.0..50.0), rng.random_range(-50.0..50.0));
                    commands.spawn((
                        crate::components::Pickup {
                            pickup_type: crate::components::PickupType::SkillGem,
                            amount: 1,
                        },
                        Sprite {
                            color: Color::srgb(0.9, 0.4, 0.8), // Magenta for skills
                            custom_size: Some(Vec2::new(16.0, 16.0)),
                            ..default()
                        },
                        Transform::from_xyz(pos.x + offset.x, pos.y + offset.y, 1.5),
                    ));

                    spawn_death_explosion(&mut commands, &assets, transform.translation);
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

/// Clean up any remaining dying effects
pub fn cleanup_dying_effects(
    mut commands: Commands,
    dying_bosses: Query<&Dying, With<Boss>>,
) {
    for dying in dying_bosses.iter() {
        if let Some(electric_entity) = dying.electric_entity {
            commands.entity(electric_entity).despawn();
        }
    }
}
