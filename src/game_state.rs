use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default] 
    Splash, // game title screen we can make something cool with animation ! later one...
    NameEntry, // enter your name screen
    Menu, // main menu screen
    Game, // gameplay screen
}