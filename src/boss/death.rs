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
