use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

pub fn default_input_map() -> InputMap<PlayerAction> {
    InputMap::new([
        (PlayerAction::MoveUp, KeyCode::KeyW),
        (PlayerAction::MoveUp, KeyCode::ArrowUp),
        (PlayerAction::MoveDown, KeyCode::KeyS),
        (PlayerAction::MoveDown, KeyCode::ArrowDown),
        (PlayerAction::MoveLeft, KeyCode::KeyA),
        (PlayerAction::MoveLeft, KeyCode::ArrowLeft),
        (PlayerAction::MoveRight, KeyCode::KeyD),
        (PlayerAction::MoveRight, KeyCode::ArrowRight),
    ])
}
