use bevy::prelude::*;
use crate::components::{Player, PlayerStats, Enemy, Health, CombatStats, PlayerEquipment};
use rand::RngExt;

#[derive(Component)]
pub struct SpinBlade {
    pub angle: f32,
    pub speed: f32,
    pub radius: f32,
    pub damage: u32,
}

#[derive(Component)]
pub struct SpinBladeTimer {
    pub reset_timer: Timer,
}

#[derive(Component)]
pub struct BurstSkillTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct BurstVisualEffect {
    pub timer: Timer,
}

pub fn animate_gem_skills(
    time: Res<Time>,
    mut materials: ResMut<Assets<crate::materials::GemSkillMaterial>>,
    mut queries: Query<&bevy::sprite_render::MeshMaterial2d<crate::materials::GemSkillMaterial>>,
    mut bursts: Query<(Entity, &mut BurstVisualEffect)>,
    mut commands: Commands,
) {
    for mat_handle in queries.iter() {
        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            mat.time += time.delta_secs();
        }
    }
    
    for (ent, mut burst) in bursts.iter_mut() {
        burst.timer.tick(time.delta());
        if burst.timer.just_finished() {
            commands.entity(ent).despawn();
        }
    }
}

pub fn update_skills(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gem_materials: ResMut<Assets<crate::materials::GemSkillMaterial>>,
    player_query: Query<(Entity, &Transform, &PlayerStats, &CombatStats), (With<Player>, Without<Enemy>)>,
    mut blades_query: Query<(Entity, &mut Transform, &mut SpinBlade), (Without<Player>, Without<Enemy>)>,
    mut enemies_query: Query<(Entity, &Transform, &mut Health), (With<Enemy>, Without<Player>)>,
    mut timer_query: Query<&mut SpinBladeTimer>,
    mut burst_query: Query<&mut BurstSkillTimer>,
    equipment: Res<PlayerEquipment>,
) {
    let mut rng = rand::rng();

    if let Some((player_entity, player_transform, stats, combat_stats)) = player_query.iter().next() {
        let player_pos = player_transform.translation.xy();
        let total_stats = equipment.get_total_stats(combat_stats);
        let magic_atk = total_stats.intelligence * 2;

        // Skill 1: Spin Blades
        if equipment.has_skill(crate::components::PlayerSkill::SpinBlades) {
            // Spawn blades if they don't exist
            if blades_query.is_empty() {
                for i in 0..3 {
                    let angle = i as f32 * std::f32::consts::TAU / 3.0;
                    commands.spawn((
                        SpinBlade {
                            angle,
                            speed: 3.0,
                            radius: 80.0,
                            damage: 5,
                        },
                        Mesh2d(meshes.add(Rectangle::new(30.0, 30.0))),
                        MeshMaterial2d(gem_materials.add(crate::materials::GemSkillMaterial {
                            color: LinearRgba::new(0.4, 0.8, 1.0, 1.0),
                            time: 0.0,
                            speed: 2.0,
                            intensity: 2.0,
                        })),
                        Transform::from_xyz(player_pos.x, player_pos.y, 1.5),
                    ));
                }
                
                commands.entity(player_entity).insert(SpinBladeTimer {
                    reset_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                });
            }

            // Update blades
            for (_blade_entity, mut blade_transform, mut blade) in blades_query.iter_mut() {
                blade.angle += blade.speed * time.delta_secs();
                let x = player_pos.x + blade.radius * blade.angle.cos();
                let y = player_pos.y + blade.radius * blade.angle.sin();
                blade_transform.translation.x = x;
                blade_transform.translation.y = y;

                // Collision with enemies (basic)
                for (_enemy_entity, enemy_transform, mut health) in enemies_query.iter_mut() {
                    if enemy_transform.translation.distance(blade_transform.translation) < 30.0 {
                        if let Some(mut timer) = timer_query.iter_mut().next() {
                            timer.reset_timer.tick(time.delta());
                            if timer.reset_timer.just_finished() {
                                let is_crit = rng.random_range(0.0..1.0) < total_stats.crit_rate;
                                // Base magic damage scaled by magic_atk (which is INT * 2, so magic_atk/2 = INT = 100% INT)
                                let mut blade_dmg = blade.damage + total_stats.intelligence;
                                if is_crit {
                                    blade_dmg = (blade_dmg as f32 * total_stats.crit_damage) as u32;
                                }

                                health.take_damage(blade_dmg);
                                crate::combat::damage::spawn_damage_number(&mut commands, blade_dmg, enemy_transform.translation.xy());
                            }
                        }
                    }
                }
            }
        } else {
            // If they dropped a level or we reset
            for (entity, _, _) in blades_query.iter() {
                commands.entity(entity).despawn();
            }
        }

        // Skill 2: Holy Burst
        if equipment.has_skill(crate::components::PlayerSkill::HolyBurst) {
            if burst_query.is_empty() {
                commands.entity(player_entity).insert(BurstSkillTimer {
                    timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                });
            } else if let Some(mut burst) = burst_query.iter_mut().next() {
                burst.timer.tick(time.delta());
                if burst.timer.just_finished() {
                    // Constant aura pulsing damage to all nearby enemies
                    let burst_radius = 180.0;
                    
                    for (_enemy_entity, enemy_transform, mut health) in enemies_query.iter_mut() {
                        if enemy_transform.translation.distance(player_transform.translation) < burst_radius {
                            let is_crit = rng.random_range(0.0..1.0) < total_stats.crit_rate;
                            let mut burst_damage = 10 + (total_stats.intelligence as f32 * 1.5) as u32;
                            if is_crit {
                                burst_damage = (burst_damage as f32 * total_stats.crit_damage) as u32;
                            }

                            health.take_damage(burst_damage);
                            crate::combat::damage::spawn_damage_number(&mut commands, burst_damage, enemy_transform.translation.xy());
                        }
                    }

                    // Visual burst effect pulse
                    commands.spawn((
                        Mesh2d(meshes.add(Circle::new(burst_radius))),
                        MeshMaterial2d(gem_materials.add(crate::materials::GemSkillMaterial {
                            color: LinearRgba::new(1.0, 0.8, 0.2, 0.6),
                            time: 0.0,
                            speed: 3.0,
                            intensity: 2.5,
                        })),
                        Transform::from_xyz(player_pos.x, player_pos.y, 1.1),
                        BurstVisualEffect { timer: Timer::from_seconds(0.3, TimerMode::Once) },
                    ));
                }
            }
        } else {
            commands.entity(player_entity).remove::<BurstSkillTimer>();
        }
    }
}

