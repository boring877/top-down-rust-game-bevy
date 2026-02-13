mod game_state;
mod screens;

use bevy::prelude::*;
use game_state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_plugins(screens::splash::splash_plugin)
        .add_plugins(screens::name_entry::name_entry_plugin)
        .run();
}
