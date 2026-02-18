use crate::components::*;
use crate::constants::*;
use crate::game_state::GameState;
use bevy::prelude::*;

pub fn spawn_toast(commands: &mut Commands, message: &str) {
    commands.spawn((
        DespawnOnExit(GameState::Menu),
        ToastNotification {
            timer: Timer::from_seconds(TOAST_DURATION, TimerMode::Once),
        },
        Node {
            position_type: PositionType::Absolute,
            bottom: px(NAVBAR_HEIGHT + 20.0),
            left: percent(50),
            margin: UiRect::left(px(-150.0)),
            width: px(300.0),
            height: px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::px(10.0, 10.0, 10.0, 10.0),
            ..default()
        },
        BackgroundColor(COLOR_TOAST_BG),
        children![
            Text::new(message),
            TextFont { font_size: FONT_SIZE_TOAST, ..default() },
            TextColor(COLOR_TEXT),
        ],
    ));
}

pub fn update_toast(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ToastNotification)>,
) {
    for (entity, mut toast) in query.iter_mut() {
        toast.timer.tick(time.delta());
        if toast.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
