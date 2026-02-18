mod currency;
mod nav;
mod showcase;
mod toast;

use crate::components::*;
use crate::game_state::GameState;
use crate::boss::particles::ParticleAssets;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;

pub use currency::spawn_currency_bar;
pub use nav::*;
pub use showcase::spawn_showcase_area;
pub use toast::*;

pub fn gacha_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), setup_gacha_menu)
        .add_systems(
            Update,
            (
                handle_nav_buttons.run_if(in_state(GameState::Menu)),
                animate_glow_effects.run_if(in_state(GameState::Menu)),
                update_toast.run_if(in_state(GameState::Menu)),
            ),
        );
}

fn setup_gacha_menu(
    mut commands: Commands,
    particle_assets: Res<ParticleAssets>,
) {
    // Spawn camera with auto-cleanup
    commands.spawn((
        DespawnOnExit(GameState::Menu),
        Camera2d,
    ));

    // Main container with auto-cleanup
    commands.spawn((
        DespawnOnExit(GameState::Menu),
        GachaMenuUI,
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(crate::constants::COLOR_MENU_BG),
        children![
            // Currency bar at top
            spawn_currency_bar(),
            // Main content area (showcase)
            spawn_showcase_area(),
            // Bottom navigation bar
            spawn_nav_bar(),
        ],
    ));

    // Background particles with auto-cleanup
    commands.spawn((
        DespawnOnExit(GameState::Menu),
        ParticleEffect::new(particle_assets.menu_ambient.clone()),
        Transform::from_xyz(0.0, 0.0, -1.0),
    ));
}
