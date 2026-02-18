use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;

pub fn spawn_currency_bar() -> impl Bundle {
    (
        CurrencyBar,
        Node {
            width: percent(100),
            height: px(CURRENCY_BAR_HEIGHT),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::horizontal(px(20.0)),
            ..default()
        },
        BackgroundColor(COLOR_CURRENCY_BG),
        children![
            // Coins
            (
                CoinDisplay,
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(8.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        Node {
                            width: px(24.0),
                            height: px(24.0),
                            border_radius: BorderRadius::px(12.0, 12.0, 12.0, 12.0),
                            ..default()
                        },
                        BackgroundColor(COLOR_COIN),
                    ),
                    (
                        Text::new("0"),
                        TextFont { font_size: FONT_SIZE_CURRENCY, ..default() },
                        TextColor(COLOR_TEXT),
                    ),
                ],
            ),
            // Gems
            (
                GemDisplay,
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(8.0),
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        Node {
                            width: px(24.0),
                            height: px(24.0),
                            border_radius: BorderRadius::px(4.0, 4.0, 4.0, 4.0),
                            ..default()
                        },
                        BackgroundColor(COLOR_GEM),
                    ),
                    (
                        Text::new("0"),
                        TextFont { font_size: FONT_SIZE_CURRENCY, ..default() },
                        TextColor(COLOR_TEXT),
                    ),
                ],
            ),
        ],
    )
}
