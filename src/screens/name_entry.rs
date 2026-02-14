use crate::components::*;
use crate::constants::*;
use crate::game_state::GameState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

pub fn name_entry_plugin(app: &mut App) {
    app.insert_resource(PlayerName(String::new()))
        .add_systems(OnEnter(GameState::NameEntry), setup_name_entry)
        .add_systems(
            Update,
            (handle_keyboard_input, handle_start_button).run_if(in_state(GameState::NameEntry)),
        )
        .add_systems(OnExit(GameState::NameEntry), cleanup_name_entry);
}

fn setup_name_entry(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        NameEntryUI,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(ROW_GAP_SMALL),
            ..default()
        },
        children![
            (
                Text::new("Enter your name:"),
                TextFont {
                    font_size: FONT_SIZE_LABEL,
                    ..default()
                },
                TextColor(COLOR_TEXT),
            ),
            (
                Text::new(""),
                TextFont {
                    font_size: FONT_SIZE_LABEL,
                    ..default()
                },
                TextColor(COLOR_TEXT),
                NameText,
            ),
            (
                Button,
                Node {
                    width: px(BUTTON_WIDTH),
                    height: px(BUTTON_HEIGHT),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(COLOR_NAME_BUTTON),
                children![
                    Text::new("Start"),
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

fn cleanup_name_entry(
    mut commands: Commands,
    query: Query<Entity, With<NameEntryUI>>,
    cameras: Query<Entity, With<Camera2d>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    for cam in cameras.iter() {
        commands.entity(cam).despawn();
    }
}

fn handle_keyboard_input(
    mut events: MessageReader<KeyboardInput>,
    mut player_name: ResMut<PlayerName>,
    mut name_text: Single<&mut Text, With<NameText>>,
) {
    for event in events.read() {
        if event.state.is_pressed() {
            if event.logical_key == Key::Backspace {
                player_name.0.pop();
            } else if let Some(text) = event.text.as_ref() {
                player_name.0.push_str(&text);
            }
            **name_text = player_name.0.clone().into();
        }
    }
}

fn handle_start_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Menu);
        }
    }
}
