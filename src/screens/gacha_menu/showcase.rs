use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;

pub fn spawn_showcase_area() -> impl Bundle {
    (
        ShowcaseArea,
        Node {
            width: percent(100),
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: px(30.0),
            ..default()
        },
        children![
            // Featured Boss Area
            (
                FeaturedBoss,
                Node {
                    width: px(300.0),
                    height: px(SHOWCASE_HEIGHT),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: px(15.0),
                    border: UiRect::all(px(3.0)),
                    border_radius: BorderRadius::px(15.0, 15.0, 15.0, 15.0),
                    ..default()
                },
                BackgroundColor(COLOR_SHOWCASE_BG),
                BorderColor {
                    top: Color::srgb(0.3, 0.2, 0.2),
                    bottom: Color::srgb(0.3, 0.2, 0.2),
                    left: Color::srgb(0.3, 0.2, 0.2),
                    right: Color::srgb(0.3, 0.2, 0.2),
                },
                children![
                    (
                        Text::new("WANTED"),
                        TextFont { font_size: FONT_SIZE_SHOWCASE_TITLE, ..default() },
                        TextColor(Color::srgb(0.9, 0.3, 0.3)),
                    ),
                    (
                        Node {
                            width: px(150.0),
                            height: px(150.0),
                            border_radius: BorderRadius::px(10.0, 10.0, 10.0, 10.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.8, 0.2, 0.3)),
                    ),
                    (
                        Text::new("DARK BOSS"),
                        TextFont { font_size: FONT_SIZE_LABEL, ..default() },
                        TextColor(COLOR_TEXT),
                    ),
                    (
                        Text::new("Threat: S"),
                        TextFont { font_size: FONT_SIZE_NAV, ..default() },
                        TextColor(Color::srgb(1.0, 0.7, 0.2)),
                    ),
                ],
            ),
            // Player Preview
            (
                PlayerPreview,
                Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: px(10.0),
                    ..default()
                },
                children![
                    (
                        Node {
                            width: px(80.0),
                            height: px(80.0),
                            border_radius: BorderRadius::px(10.0, 10.0, 10.0, 10.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.6, 0.9)),
                    ),
                    (
                        Text::new("Your Fighter"),
                        TextFont { font_size: FONT_SIZE_NAV, ..default() },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ),
                ],
            ),
        ],
    )
}
