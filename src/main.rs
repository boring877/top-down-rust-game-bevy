pub mod components;
pub mod constants;
mod game_state;
mod screens;

// New modular structure
mod materials;
mod player;
mod boss;
mod combat;
mod world;

use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy::sprite_render::Material2dPlugin;
use avian2d::prelude::*;
use leafwing_input_manager::prelude::*;

use components::PlayerAction;
use game_state::GameState;
use materials::{PlayerMaterial, BossMaterial, BulletMaterial, BladeMaterial, ObstacleMaterial, FloorMaterial, GemSkillMaterial};
use boss::BossSpawnTimer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Player VS Boss".to_string(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        // Material plugins
        .add_plugins(Material2dPlugin::<PlayerMaterial>::default())
        .add_plugins(Material2dPlugin::<BossMaterial>::default())
        .add_plugins(Material2dPlugin::<BulletMaterial>::default())
        .add_plugins(Material2dPlugin::<BladeMaterial>::default())
        .add_plugins(Material2dPlugin::<ObstacleMaterial>::default())
        .add_plugins(Material2dPlugin::<FloorMaterial>::default())
        .add_plugins(Material2dPlugin::<GemSkillMaterial>::default())
        // Physics
        .add_plugins(PhysicsPlugins::default())
        // Particles
        .add_plugins(bevy_hanabi::HanabiPlugin)
        // Resources
        .init_resource::<BossSpawnTimer>()
        .init_resource::<components::PlayerEquipment>()
        // Setup particle assets at startup (needed for menu particles)
        .add_systems(Startup, boss::setup_particle_assets)
        // Spawn systems - each function checks if entities already exist
        .add_systems(
            OnEnter(GameState::Game),
            (
                boss::reset_boss_timer,
                world::spawn_camera,
                world::spawn_floor,
                player::spawn_player,
                world::spawn_obstacles,
            ),
        )
        // Update systems - only run during Game state (paused during PauseMenu)
        .add_systems(
            Update,
            (
                // Player
                player::player_movement,
                player::clamp_player_to_arena,
                player::animate_player,
                player::sync_player_stats,
                // World
                world::update_camera,
                // Boss
                boss::boss_chase_player,
                boss::clamp_boss_to_arena,
                boss::animate_boss,
                boss::test_spawn_bosses,
                boss::boss_attack,
                boss::animate_blade,
                boss::blade_collision,
                boss::enemy_death,
                boss::animate_boss_death,
                // Combat
                combat::spawn_bullet,
                combat::bullet_movement_and_collision,
                combat::animate_damage_numbers,
                combat::update_skills,
                combat::animate_gem_skills,
                world::update_pickups,
            ).run_if(in_state(GameState::Game)),
        )
        // Cleanup systems - only run when going to Menu
        .add_systems(
            OnEnter(GameState::Menu),
            (
                player::cleanup_player,
                boss::cleanup_boss,
                boss::cleanup_particles,
                boss::cleanup_dying_effects,
                world::cleanup_floor,
                world::cleanup_camera,
                combat::cleanup_bullets,
                world::cleanup_obstacles,
                world::cleanup_pickups,
            ),
        )
        // Screens
        .add_plugins(screens::splash::splash_plugin)
        .add_plugins(screens::gacha_menu::gacha_menu_plugin)
        .add_plugins(screens::settings::settings_plugin)
        .add_plugins(screens::name_entry::name_entry_plugin)
        .add_plugins(screens::pause_menu::pause_menu_plugin)
        .add_plugins(screens::stash_menu::stash_menu_plugin)
        .add_plugins(screens::hud::hud_plugin)
        .run();
}
