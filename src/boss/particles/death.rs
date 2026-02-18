use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use super::ParticleAssets;

/// Spawn death explosion at position
pub fn spawn_death_explosion(
    commands: &mut Commands,
    assets: &ParticleAssets,
    position: Vec3,
) {
    commands.spawn((
        ParticleEffect::new(assets.death_effect.clone()),
        Transform::from_translation(position),
        Name::new("DeathExplosion"),
    ));
}

/// Spawn electricity effect around dying boss (continuous during collapse)
pub fn spawn_electric_effect(
    commands: &mut Commands,
    assets: &ParticleAssets,
    position: Vec3,
) -> Entity {
    commands.spawn((
        ParticleEffect::new(assets.electric_effect.clone()),
        Transform::from_translation(position),
        Name::new("Electricity"),
    )).id()
}
