use bevy::prelude::*;
use crate::components::{Pickup, PickupType, Player, PlayerStats, Item, ItemRarity, ItemSet};
use rand::RngExt;
use crate::combat::spawn_damage_number;

const PICKUP_MAGNET_RADIUS: f32 = 150.0;
const PICKUP_COLLECT_RADIUS: f32 = 20.0;
const PICKUP_SPEED: f32 = 400.0;

pub fn update_pickups(
    time: Res<Time>,
    mut commands: Commands,
    mut equipment: ResMut<crate::components::PlayerEquipment>,
    mut player_query: Query<(&Transform, &mut PlayerStats), With<Player>>,
    mut pickups_query: Query<(Entity, &mut Transform, &Pickup), Without<Player>>,
) {
    if let Some((player_transform, mut stats)) = player_query.iter_mut().next() {
        let player_pos = player_transform.translation.xy();

        for (entity, mut pickup_transform, pickup) in pickups_query.iter_mut() {
            let pickup_pos = pickup_transform.translation.xy();
            let distance = player_pos.distance(pickup_pos);

            // Magnet effect
            if distance < PICKUP_MAGNET_RADIUS {
                let direction = (player_pos - pickup_pos).normalize_or_zero();
                pickup_transform.translation.x += direction.x * PICKUP_SPEED * time.delta_secs();
                pickup_transform.translation.y += direction.y * PICKUP_SPEED * time.delta_secs();

                if distance < PICKUP_COLLECT_RADIUS {
                    match pickup.pickup_type {
                        PickupType::Xp => stats.add_xp(pickup.amount),
                        PickupType::Gold => stats.gold += pickup.amount,
                        PickupType::Material => stats.materials += pickup.amount,
                        PickupType::Weapon => { equipment.inventory.push(generate_item(PickupType::Weapon)); }
                        PickupType::Helmet => { equipment.inventory.push(generate_item(PickupType::Helmet)); }
                        PickupType::Armor => { equipment.inventory.push(generate_item(PickupType::Armor)); }
                        PickupType::Pants => { equipment.inventory.push(generate_item(PickupType::Pants)); }
                        PickupType::Shoes => { equipment.inventory.push(generate_item(PickupType::Shoes)); }
                        PickupType::Ring => { equipment.inventory.push(generate_item(PickupType::Ring)); }
                        PickupType::Earring => { equipment.inventory.push(generate_item(PickupType::Earring));  }
                        PickupType::Necklace => { equipment.inventory.push(generate_item(PickupType::Necklace)); }
                        PickupType::Gemstone => { equipment.inventory.push(generate_item(PickupType::Gemstone)); }
                        PickupType::SkillGem => { equipment.inventory.push(generate_item(PickupType::SkillGem)); }
                    }
                    
                    // Spawn some visual text
                    spawn_damage_number(&mut commands, pickup.amount, player_pos); // TODO: damage text only accepts u32 right now, wait, I can just leave it as pickup.amount or update spawn_damage_number.
                    
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

pub fn cleanup_pickups(mut commands: Commands, pickups: Query<Entity, With<Pickup>>) {
    for entity in pickups.iter() {
        commands.entity(entity).despawn();
    }
}

fn generate_item(pickup_type: PickupType) -> Item {
    let mut rng = rand::rng();
    
    let rarity_roll = rng.random_range(0..100);
    let rarity = if rarity_roll < 50 {
        ItemRarity::Common
    } else if rarity_roll < 80 {
        ItemRarity::Uncommon
    } else if rarity_roll < 95 {
        ItemRarity::Rare
    } else if rarity_roll < 99 {
        ItemRarity::Epic
    } else {
        ItemRarity::Legendary
    };

    let set_roll = rng.random_range(0..100);
    let set = if set_roll < 50 {
        ItemSet::None
    } else if set_roll < 65 {
        ItemSet::WarriorSet
    } else if set_roll < 80 {
        ItemSet::MageSet
    } else {
        ItemSet::RogueSet
    };

    let multiplier = match rarity {
        ItemRarity::Common => 1,
        ItemRarity::Uncommon => 2,
        ItemRarity::Rare => 3,
        ItemRarity::Epic => 5,
        ItemRarity::Legendary => 8,
    };
    
    let intelligence = rng.random_range(0..=5) * multiplier;
    let strength = rng.random_range(0..=5) * multiplier;
    let agility = rng.random_range(0..=5) * multiplier;
    let dodge_rate = (rng.random_range(1.0..5.0) * (multiplier as f32)) / 100.0;
    let crit_rate = (rng.random_range(1.0..3.0) * (multiplier as f32)) / 100.0;
    let crit_damage = (rng.random_range(5.0..15.0) * (multiplier as f32)) / 100.0;

    let base_name = match pickup_type {
        PickupType::Weapon => "Blade",
        PickupType::Helmet => "Helm",
        PickupType::Armor => "Plate",
        PickupType::Pants => "Guards",
        PickupType::Shoes => "Treads",
        PickupType::Ring => "Band",
        PickupType::Earring => "Stud",
        PickupType::Necklace => "Amulet",
        PickupType::Gemstone => "Facet",
        PickupType::SkillGem => "Skill Gem",
        _ => "Relic",
    };

    let granted_skill = if pickup_type == PickupType::SkillGem {
        use crate::components::PlayerSkill;
        let skill_roll = rng.random_range(0..5);
        Some(match skill_roll {
            0 => PlayerSkill::SpinBlades,
            1 => PlayerSkill::HolyBurst,
            2 => PlayerSkill::Fireball,
            3 => PlayerSkill::Dash,
            _ => PlayerSkill::Earthquake,
        })
    } else {
        None
    };

    Item {
        name: format!("{:?} {}", rarity, base_name),
        rarity,
        set,
        pickup_type,
        intelligence,
        strength,
        agility,
        dodge_rate,
        crit_rate,
        crit_damage,
        granted_skill,
    }
}
