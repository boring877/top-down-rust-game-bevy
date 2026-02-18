use crate::components::*;
use crate::constants::*;
use crate::game_state::GameState;
use crate::screens::settings::PreviousState;
use bevy::prelude::*;

pub fn pause_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::PauseMenu), (setup_pause_menu, pause_game))
        .add_systems(OnExit(GameState::PauseMenu), resume_game)
        .add_systems(
            Update,
            (
                handle_pause_buttons,
                handle_pause_input,
            ).run_if(in_state(GameState::PauseMenu)),
        )
        .add_systems(
            Update,
            handle_escape_key.run_if(in_state(GameState::Game)),
        );
}

fn pause_game(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

fn resume_game(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}

fn setup_pause_menu(mut commands: Commands) {
    // Dark overlay popup with auto-cleanup
    commands.spawn((
        DespawnOnExit(GameState::PauseMenu),
        PauseMenuUI,
        Node {
            width: percent(100),
            height: percent(100),
            position_type: PositionType::Absolute,
            left: px(0.0),
            top: px(0.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        children![
            // Menu box container
            (
                Node {
                    width: px(300.0),
                    padding: UiRect::all(px(30.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(ROW_GAP_LARGE),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.95)),
                children![
                    (
                        Text::new("PAUSED"),
                        TextFont {
                            font_size: FONT_SIZE_TITLE,
                            ..default()
                        },
                        TextColor(COLOR_TEXT),
                    ),
                    (
                        Button,
                        ResumeButton,
                        Node {
                            width: px(BUTTON_WIDTH),
                            height: px(BUTTON_HEIGHT),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(COLOR_START_BUTTON),
                        children![
                            Text::new("Resume"),
                            TextFont {
                                font_size: FONT_SIZE_BUTTON,
                                ..default()
                            },
                            TextColor(COLOR_TEXT),
                        ]
                    ),
                    (
                        Button,
                        ResetButton,
                        Node {
                            width: px(BUTTON_WIDTH),
                            height: px(BUTTON_HEIGHT),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.5, 0.4, 0.2)),
                        children![
                            Text::new("Reset"),
                            TextFont {
                                font_size: FONT_SIZE_BUTTON,
                                ..default()
                            },
                            TextColor(COLOR_TEXT),
                        ]
                    ),
                    (
                        Button,
                        SettingsButton,
                        Node {
                            width: px(BUTTON_WIDTH),
                            height: px(BUTTON_HEIGHT),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(COLOR_SETTINGS_BUTTON),
                        children![
                            Text::new("Settings"),
                            TextFont {
                                font_size: FONT_SIZE_BUTTON,
                                ..default()
                            },
                            TextColor(COLOR_TEXT),
                        ]
                    ),
                    (
                        Button,
                        MainMenuButton,
                        Node {
                            width: px(BUTTON_WIDTH),
                            height: px(BUTTON_HEIGHT),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(COLOR_BACK_BUTTON),
                        children![
                            Text::new("Main Menu"),
                            TextFont {
                                font_size: FONT_SIZE_BUTTON,
                                ..default()
                            },
                            TextColor(COLOR_TEXT),
                        ]
                    )
                ]
            )
        ],
    ));
}

fn handle_pause_buttons(
    resume_query: Query<&Interaction, With<ResumeButton>>,
    reset_query: Query<&Interaction, With<ResetButton>>,
    settings_query: Query<&Interaction, (With<SettingsButton>, Without<ResumeButton>, Without<ResetButton>, Without<MainMenuButton>)>,
    main_menu_query: Query<&Interaction, With<MainMenuButton>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut previous_state: ResMut<PreviousState>,
    mut commands: Commands,
    players: Query<Entity, With<crate::components::Player>>,
    bosses: Query<Entity, With<crate::components::Boss>>,
    bullets: Query<Entity, With<crate::components::Bullet>>,
    floors: Query<Entity, With<crate::components::Floor>>,
    cameras: Query<Entity, With<Camera2d>>,
    obstacles: Query<Entity, With<crate::components::Obstacle>>,
) {
    // Resume - just close the menu
    for interaction in resume_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Game);
        }
    }

    // Reset - cleanup and restart game
    for interaction in reset_query.iter() {
        if *interaction == Interaction::Pressed {
            // Cleanup all game entities
            for entity in players.iter() {
                commands.entity(entity).despawn();
            }
            for entity in bosses.iter() {
                commands.entity(entity).despawn();
            }
            for entity in bullets.iter() {
                commands.entity(entity).despawn();
            }
            for entity in floors.iter() {
                commands.entity(entity).despawn();
            }
            for entity in cameras.iter() {
                commands.entity(entity).despawn();
            }
            for entity in obstacles.iter() {
                commands.entity(entity).despawn();
            }
            // Note: We keep ParticleAssets since they're just effect templates,
            // not active effects. They're needed for the menu particles.
            // Go back to Game (will trigger OnEnter to respawn)
            next_state.set(GameState::Game);
        }
    }

    // Settings - go to settings menu (game stays in background)
    for interaction in settings_query.iter() {
        if *interaction == Interaction::Pressed {
            previous_state.0 = GameState::PauseMenu;
            next_state.set(GameState::SettingsMenu);
        }
    }

    // Main Menu - cleanup and go to menu
    for interaction in main_menu_query.iter() {
        if *interaction == Interaction::Pressed {
            // Cleanup happens via OnEnter(Menu) system
            next_state.set(GameState::Menu);
        }
    }
}

fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Game);
    }
}

fn handle_escape_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::PauseMenu);
    }
}
