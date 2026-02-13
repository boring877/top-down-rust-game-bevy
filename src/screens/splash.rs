use bevy::prelude::*;
use crate::game_state::GameState;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

#[derive(Component)]
struct SplashUI;

pub fn splash_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), cleanup_splash);
}

fn splash_setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        SplashUI,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Text::new("Player VS Boss"),
            TextFont {
                font_size: 80.0,
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    ));

    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
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

fn cleanup_splash(
    mut commands: Commands,
    query: Query<Entity, With<SplashUI>>,
    cameras: Query<Entity, With<Camera2d>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    for cam in cameras.iter() {
        commands.entity(cam).despawn();
    }
}
