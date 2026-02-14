use crate::components::*;
use crate::constants::*;
use crate::game_state::GameState;
use bevy::prelude::*;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(
            Update,
            handle_menu_buttons.run_if(in_state(GameState::Menu)),
        )
        .add_systems(OnExit(GameState::Menu), cleanup_menu);
}

fn setup_menu(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        MenuUI,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(ROW_GAP_LARGE),
            ..default()
        },
        children![
            (
                Text::new(GAME_TITLE),
                TextFont {
                    font_size: FONT_SIZE_TITLE,
                    ..default()
                },
                TextColor(COLOR_TEXT),
            ),
            (
                Button,
                StartButton,
                Node {
                    width: px(BUTTON_WIDTH),
                    height: px(BUTTON_HEIGHT),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(COLOR_START_BUTTON),
                children![
                    Text::new("Start"),
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
            )
        ],
    ));
}

fn cleanup_menu(
    mut commands: Commands,
    query: Query<Entity, With<MenuUI>>,
    cameras: Query<Entity, With<Camera2d>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    for cam in cameras.iter() {
        commands.entity(cam).despawn();
    }
}

fn handle_menu_buttons(
    start_query: Query<&Interaction, With<StartButton>>,
    settings_query: Query<&Interaction, With<SettingsButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in start_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Game);
        }
    }
    for interaction in settings_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::SettingsMenu);
        }
    }
}
