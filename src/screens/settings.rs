use crate::components::*;
use crate::constants::*;
use crate::game_state::GameState;
use bevy::prelude::*;

// Track where we came from to return there
#[derive(Resource, Default)]
pub struct PreviousState(pub GameState);

#[derive(Component)]
struct SettingsCamera;

pub fn settings_plugin(app: &mut App) {
    app.init_resource::<PreviousState>()
        .add_systems(OnEnter(GameState::SettingsMenu), setup_settings)
        .add_systems(
            Update,
            handle_back_button.run_if(in_state(GameState::SettingsMenu)),
        );
}

fn setup_settings(mut commands: Commands, previous: Res<PreviousState>) {
    // Only spawn a new camera if coming from Menu (not from pause/game)
    if previous.0 == GameState::Menu {
        commands.spawn((
            DespawnOnExit(GameState::SettingsMenu),
            Camera2d,
            SettingsCamera,
        ));
    }

    // Settings UI as overlay with auto-cleanup
    commands.spawn((
        DespawnOnExit(GameState::SettingsMenu),
        SettingsUI,
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
            // Settings box
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
                ]
            )
        ],
    ));
}

fn handle_back_button(
    interaction_query: Query<&Interaction, With<BackButton>>,
    mut next_state: ResMut<NextState<GameState>>,
    previous: Res<PreviousState>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(previous.0);
        }
    }
}
