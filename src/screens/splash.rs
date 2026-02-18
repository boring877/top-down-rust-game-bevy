use crate::components::*;
use crate::constants::*;
use crate::game_state::GameState;
use bevy::prelude::*;

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)));
}

fn splash_setup(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Splash),
        Camera2d,
    ));

    commands.spawn((
        DespawnOnExit(GameState::Splash),
        SplashUI,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Text::new(GAME_TITLE),
            TextFont {
                font_size: FONT_SIZE_SPLASH,
                ..default()
            },
            TextColor(COLOR_TEXT),
        )],
    ));

    commands.insert_resource(SplashTimer(Timer::from_seconds(
        SPLASH_DURATION_SECS,
        TimerMode::Once,
    )));
}

fn countdown(
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if timer.tick(time.delta()).just_finished() {
        next_state.set(GameState::NameEntry);
    }
}
