use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Splash,
    NameEntry,
    Menu,
    SettingsMenu,
    Game,
    PauseMenu,  // Pause/reset menu during gameplay
}
