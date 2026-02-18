pub mod death;

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

#[derive(Resource)]
pub struct ParticleAssets {
    pub death_effect: Handle<EffectAsset>,
    pub electric_effect: Handle<EffectAsset>,
    pub menu_ambient: Handle<EffectAsset>,
}

pub fn setup_particle_assets(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    existing_assets: Option<Res<ParticleAssets>>,
) {
    // Don't recreate assets if they already exist (e.g., returning from pause)
    if existing_assets.is_some() {
        return;
    }

    // === DEATH EXPLOSION EFFECT ===
    let mut death_gradient = bevy_hanabi::Gradient::new();
    death_gradient.add_key(0.0, Vec4::new(1.0, 0.8, 0.2, 1.0));
    death_gradient.add_key(0.2, Vec4::new(1.0, 0.4, 0.1, 0.9));
    death_gradient.add_key(0.6, Vec4::new(0.8, 0.2, 0.1, 0.6));
    death_gradient.add_key(1.0, Vec4::new(0.3, 0.1, 0.05, 0.0));

    let death_writer = ExprWriter::new();
    let death_center = death_writer.lit(Vec3::ZERO).expr();
    let death_axis = death_writer.lit(Vec3::Z).expr();
    let death_radius = death_writer.lit(30.0).expr();
    let death_speed = death_writer.lit(200.0).expr();
    let death_lifetime = death_writer.lit(0.8).expr();
    let death_age = death_writer.lit(0.0).expr();
    let death_size = Vec3::splat(12.0).into();
    let death_spawner = SpawnerSettings::once(80.0.into());

    let death_effect = effects.add(
        EffectAsset::new(512, death_spawner, death_writer.finish())
            .with_name("DeathExplosion")
            .init(SetPositionCircleModifier {
                center: death_center,
                axis: death_axis,
                radius: death_radius,
                dimension: ShapeDimension::Surface,
            })
            .init(SetVelocityCircleModifier {
                center: death_center,
                axis: death_axis,
                speed: death_speed,
            })
            .init(SetAttributeModifier::new(Attribute::LIFETIME, death_lifetime))
            .init(SetAttributeModifier::new(Attribute::AGE, death_age))
            .render(SetSizeModifier { size: death_size })
            .render(ColorOverLifetimeModifier::new(death_gradient)),
    );

    // === ELECTRICITY EFFECT (for collapsing phase) ===
    let mut electric_gradient = bevy_hanabi::Gradient::new();
    electric_gradient.add_key(0.0, Vec4::new(0.5, 0.7, 1.0, 1.0));   // Bright cyan/blue
    electric_gradient.add_key(0.3, Vec4::new(0.8, 0.9, 1.0, 0.8));   // White-ish
    electric_gradient.add_key(0.7, Vec4::new(0.3, 0.5, 1.0, 0.4));   // Blue
    electric_gradient.add_key(1.0, Vec4::new(0.1, 0.2, 0.5, 0.0));   // Fade out

    let electric_writer = ExprWriter::new();
    let electric_center = electric_writer.lit(Vec3::ZERO).expr();
    let electric_axis = electric_writer.lit(Vec3::Z).expr();
    let electric_radius = electric_writer.lit(50.0).expr();
    let electric_speed = electric_writer.lit(100.0).expr();
    let electric_lifetime = electric_writer.lit(0.3).expr();
    let electric_age = electric_writer.lit(0.0).expr();
    let electric_size = Vec3::splat(4.0).into();
    let electric_spawner = SpawnerSettings::rate(60.0.into()); // Continuous during collapse

    let electric_effect = effects.add(
        EffectAsset::new(256, electric_spawner, electric_writer.finish())
            .with_name("Electricity")
            .init(SetPositionCircleModifier {
                center: electric_center,
                axis: electric_axis,
                radius: electric_radius,
                dimension: ShapeDimension::Surface,
            })
            .init(SetVelocityCircleModifier {
                center: electric_center,
                axis: electric_axis,
                speed: electric_speed,
            })
            .init(SetAttributeModifier::new(Attribute::LIFETIME, electric_lifetime))
            .init(SetAttributeModifier::new(Attribute::AGE, electric_age))
            .render(SetSizeModifier { size: electric_size })
            .render(ColorOverLifetimeModifier::new(electric_gradient)),
    );

    // === MENU AMBIENT EFFECT (floating particles for gacha menu) ===
    let mut menu_gradient = bevy_hanabi::Gradient::new();
    menu_gradient.add_key(0.0, Vec4::new(0.6, 0.5, 0.9, 0.8));   // Purple
    menu_gradient.add_key(0.5, Vec4::new(0.9, 0.8, 0.4, 0.6));   // Gold
    menu_gradient.add_key(1.0, Vec4::new(0.4, 0.6, 0.9, 0.0));   // Blue fade

    let menu_writer = ExprWriter::new();
    let menu_age = menu_writer.lit(0.0).expr();
    let menu_lifetime = menu_writer.lit(4.0).expr();
    let menu_size = Vec3::splat(6.0).into();
    let menu_spawner = SpawnerSettings::rate(15.0.into());

    // Random position across screen
    let menu_pos = menu_writer.lit(Vec3::ZERO).expr();
    let menu_vel = menu_writer.lit(Vec3::new(0.0, 30.0, 0.0)).expr();
    let menu_radius = menu_writer.lit(500.0).expr();

    let menu_ambient = effects.add(
        EffectAsset::new(256, menu_spawner, menu_writer.finish())
            .with_name("MenuAmbient")
            .init(SetAttributeModifier::new(Attribute::AGE, menu_age))
            .init(SetAttributeModifier::new(Attribute::LIFETIME, menu_lifetime))
            .init(SetPositionSphereModifier {
                center: menu_pos,
                radius: menu_radius,
                dimension: ShapeDimension::Volume,
            })
            .init(SetAttributeModifier::new(Attribute::VELOCITY, menu_vel))
            .render(SetSizeModifier { size: menu_size })
            .render(ColorOverLifetimeModifier::new(menu_gradient)),
    );

    commands.insert_resource(ParticleAssets {
        death_effect,
        electric_effect,
        menu_ambient,
    });
}

pub fn cleanup_particles(mut commands: Commands, effects: Query<Entity, With<ParticleEffect>>) {
    for entity in effects.iter() {
        commands.entity(entity).despawn();
    }
}

// Re-export from submodules
pub use death::spawn_death_explosion;
pub use death::spawn_electric_effect;
