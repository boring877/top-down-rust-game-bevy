use crate::components::*;
use crate::constants::*;
use crate::game_state::GameState;
use bevy::prelude::*;

pub fn settings_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::SettingsMenu), setup_settings)
        .add_systems(
            Update,
            handle_back_button.run_if(in_state(GameState::SettingsMenu)),
        )
        .add_systems(OnExit(GameState::SettingsMenu), cleanup_settings);
}

fn setup_settings(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        SettingsUI,
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
                Text::new("Settings"),
                TextFont {
                    font_size: FONT_SIZE_TITLE,
                    ..default()
                },
                TextColor(COLOR_TEXT),
            ),
            (
                Button,
                BackButton,
                Node {
                    width: px(BUTTON_WIDTH),
                    height: px(BUTTON_HEIGHT),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(COLOR_BACK_BUTTON),
                children![
                    Text::new("Back"),
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

fn cleanup_settings(
    mut commands: Commands,
    query: Query<Entity, With<SettingsUI>>,
    cameras: Query<Entity, With<Camera2d>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    for cam in cameras.iter() {
        commands.entity(cam).despawn();
    }
}

fn handle_back_button(
    interaction_query: Query<&Interaction, With<BackButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Menu);
        }
    }
}
