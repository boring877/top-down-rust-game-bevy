use crate::components::{Health, Player, FireRate, ShotCounter, PlayerStats};
use crate::materials::PlayerMaterial;
use crate::constants::*;
use bevy::prelude::*;
use bevy::sprite_render::MeshMaterial2d;
use avian2d::prelude::*;
use crate::components::default_input_map;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    existing_players: Query<(), With<Player>>,
) {
    // Don't spawn if player already exists (e.g., returning from pause)
    if !existing_players.is_empty() {
        return;
    }

    commands.spawn((
        FireRate::new(DEFAULT_FIRE_RATE),
        ShotCounter::default(),
        Player,
        RigidBody::Dynamic,
        Collider::circle(PLAYER_SIZE.x / 2.0),
        LinearDamping(5.0),
        AngularDamping(10.0),
        Sprite {
            image: asset_server.load("sprites/hero.png"),
            custom_size: Some(Vec2::new(PLAYER_SIZE.x * 6.0, PLAYER_SIZE.y * 6.0)),
            texture_atlas: Some(TextureAtlas {
                layout: layouts.add(TextureAtlasLayout::from_grid(UVec2::new(192, 192), 6, 1, None, None)),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        default_input_map(),
        Health::new(PLAYER_HEALTH),
        PlayerStats::default(),
        crate::components::CombatStats::default(),
    ));
}

pub fn sync_player_stats(
    mut player_query: Query<(&crate::components::CombatStats, &mut Health), With<Player>>,
    equipment: Res<crate::components::PlayerEquipment>,
) {
    if equipment.is_changed() {
        for (base_stats, mut health) in player_query.iter_mut() {
            let total = equipment.get_total_stats(base_stats);
            // 1 STR = 5 HP. Subtract base_stats.strength so the base hp covers the initial 10 STR.
            let hp_bonus = (total.strength.saturating_sub(base_stats.strength)) * 5;
            let new_max = PLAYER_HEALTH + hp_bonus;
            
            if health.max != new_max {
                let diff = new_max as i32 - health.max as i32;
                health.max = new_max;
                if diff > 0 {
                    health.current = health.current.saturating_add(diff as u32);
                } else {
                    health.current = health.current.min(health.max);
                }
            }
        }
    }
}

pub fn cleanup_player(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for entity in players.iter() {
        commands.entity(entity).despawn();
    }
}
