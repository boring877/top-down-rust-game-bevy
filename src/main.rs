pub mod components;
pub mod constants;
mod game;
mod game_state;
mod screens;

use bevy::prelude::*;
use bevy::window::WindowMode;
use components::PlayerAction;
use game_state::GameState;
use leafwing_input_manager::prelude::*;

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
        .add_plugins(game::game_plugin)
        .add_plugins(screens::splash::splash_plugin)
        .add_plugins(screens::menu::menu_plugin)
        .add_plugins(screens::settings::settings_plugin)
        .add_plugins(screens::name_entry::name_entry_plugin)
        .run();
}
