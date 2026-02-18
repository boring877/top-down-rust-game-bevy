use crate::components::*;
use crate::constants::*;
use crate::game_state::GameState;
use crate::screens::settings::PreviousState;
use super::{ToastNotification, spawn_toast};
use bevy::prelude::*;

pub fn spawn_nav_bar() -> impl Bundle {
    (
        NavBar,
        Node {
            width: percent(100),
            height: px(NAVBAR_HEIGHT),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(COLOR_NAVBAR_BG),
        children![
            // Home button
            spawn_nav_button("Home", HomeButton),
            // Shop button (placeholder)
            spawn_nav_button("Shop", ShopButton),
            // PLAY button (center, prominent)
            spawn_play_button(),
            // Collection button (placeholder)
            spawn_nav_button("Collection", CollectionButton),
            // Settings button
            spawn_nav_button("Settings", GachaSettingsButton),
        ],
    )
}

fn spawn_nav_button<T: Component>(label: &str, _marker: T) -> impl Bundle {
    (
        Button,
        NavButton,
        _marker,
        Node {
            width: px(NAV_BUTTON_SIZE),
            height: px(NAV_BUTTON_SIZE),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::px(10.0, 10.0, 10.0, 10.0),
            ..default()
        },
        BackgroundColor(COLOR_NAV_BUTTON),
        children![
            Text::new(label),
            TextFont { font_size: FONT_SIZE_NAV, ..default() },
            TextColor(COLOR_TEXT),
        ],
    )
}

fn spawn_play_button() -> impl Bundle {
    (
        Button,
        NavButton,
        PlayButton,
        GlowEffect {
            base_color: COLOR_PLAY_BUTTON,
            glow_color: COLOR_GLOW,
            time: 0.0,
        },
        Node {
            width: px(PLAY_BUTTON_SIZE),
            height: px(PLAY_BUTTON_SIZE),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::px(PLAY_BUTTON_SIZE / 2.0, PLAY_BUTTON_SIZE / 2.0, PLAY_BUTTON_SIZE / 2.0, PLAY_BUTTON_SIZE / 2.0),
            ..default()
        },
        BackgroundColor(COLOR_PLAY_BUTTON),
        children![
            Text::new("PLAY"),
            TextFont { font_size: FONT_SIZE_BUTTON, ..default() },
            TextColor(COLOR_TEXT),
        ],
    )
}

pub fn handle_nav_buttons(
    mut commands: Commands,
    play_query: Query<&Interaction, With<PlayButton>>,
    shop_query: Query<&Interaction, (With<ShopButton>, Without<ToastNotification>)>,
    collection_query: Query<&Interaction, (With<CollectionButton>, Without<ToastNotification>)>,
    settings_query: Query<&Interaction, With<GachaSettingsButton>>,
    coin_query: Query<&Interaction, With<CoinDisplay>>,
    gem_query: Query<&Interaction, With<GemDisplay>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut previous_state: ResMut<PreviousState>,
    existing_toast: Query<Entity, With<ToastNotification>>,
) {
    // Play button -> Start game
    for interaction in play_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Game);
        }
    }

    // Settings button
    for interaction in settings_query.iter() {
        if *interaction == Interaction::Pressed {
            previous_state.0 = GameState::Menu;
            next_state.set(GameState::SettingsMenu);
        }
    }

    // Placeholder buttons - show toast (handle each query separately)
    let should_show_toast = shop_query.iter().any(|i| *i == Interaction::Pressed)
        || collection_query.iter().any(|i| *i == Interaction::Pressed)
        || coin_query.iter().any(|i| *i == Interaction::Pressed)
        || gem_query.iter().any(|i| *i == Interaction::Pressed);

    if should_show_toast && existing_toast.is_empty() {
        spawn_toast(&mut commands, "Coming Soon!");
    }
}

pub fn animate_glow_effects(
    time: Res<Time>,
    mut query: Query<(&mut GlowEffect, &mut BackgroundColor), With<PlayButton>>,
) {
    for (mut glow, mut bg_color) in query.iter_mut() {
        glow.time += time.delta_secs() * 3.0;
        let pulse = (glow.time.sin() + 1.0) * 0.5; // 0.0 to 1.0

        // Lerp between base color and glow color
        let base = glow.base_color.to_srgba();
        let glow_c = glow.glow_color.to_srgba();

        let r = base.red + (glow_c.red - base.red) * pulse * 0.3;
        let g = base.green + (glow_c.green - base.green) * pulse * 0.3;
        let b = base.blue + (glow_c.blue - base.blue) * pulse * 0.3;

        bg_color.0 = Color::srgba(r, g, b, 1.0);
    }
}
