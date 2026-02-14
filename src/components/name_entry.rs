use bevy::prelude::*;

#[derive(Component)]
pub struct NameText;

#[derive(Component)]
pub struct NameEntryUI;

#[derive(Resource)]
pub struct PlayerName(pub String);
