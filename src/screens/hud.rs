use crate::game_state::GameState;
use crate::components::{Player, Health, PlayerStats};
use bevy::prelude::*;

pub fn hud_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), spawn_hud)
       .add_systems(Update, (handle_hud_buttons, update_hud_stats).run_if(in_state(GameState::Game)))
       .add_systems(OnExit(GameState::Game), cleanup_hud);
}

#[derive(Component)]
struct HudUI;

#[derive(Component)]
struct BagButton;

#[derive(Component)]
struct HealthBarFill;

#[derive(Component)]
struct HealthText;

#[derive(Component)]
struct SkillsUI;

#[derive(Component)]
struct HudSkillSlot(usize);

#[derive(Component)]
struct HudSkillText;

fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        HudUI,
        Node {
            position_type: PositionType::Absolute,
            right: px(20.0),
            top: px(20.0),
            width: px(80.0),
            height: px(80.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
    )).with_children(|parent| {
        parent.spawn((
            Button,
            BagButton,
            Node {
                width: percent(100.0),
                height: percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.3, 0.25, 0.2, 0.9)),
        )).with_children(|btn| {
            btn.spawn((
                Text::new("BAG\n[Tab]"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(1.0, 0.9, 0.6)),
                TextLayout::new_with_justify(Justify::Center),
            ));
        });
    });

    // Bottom HUD for Health and Skills
    commands.spawn((
        HudUI,
        Node {
            position_type: PositionType::Absolute,
            bottom: px(20.0),
            left: percent(50.0),
            width: px(600.0),
            height: px(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            margin: UiRect::left(px(-300.0)), // Center trick
            padding: UiRect::all(px(15.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
    )).with_children(|bottom| {
        // Left: HP bar
        bottom.spawn(Node {
            width: px(250.0),
            height: px(35.0),
            border: UiRect::all(px(0.0)),
            ..default()
        })
        .with_children(|hp_box| {
            hp_box.spawn((
                HealthBarFill,
                Node {
                    width: percent(100.0),
                    height: percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.9, 0.2, 0.2)),
            ));
            hp_box.spawn((
                HealthText,
                Text::new("HP: 100/100"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    left: px(10.0),
                    top: px(5.0),
                    ..default()
                }
            ));
        });

        // Right: Skills UI nodes with slots
        bottom.spawn((
            SkillsUI,
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: px(10.0),
                ..default()
            }
        )).with_children(|skills_row| {
            for i in 1..=4 {
                skills_row.spawn((
                    HudSkillSlot(i),
                    Node {
                        width: px(55.0),
                        height: px(55.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
                )).with_children(|slot| {
                    // Slot hotkey label (e.g. 1, 2, 3, 4)
                    slot.spawn((
                        Text::new(i.to_string()),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgb(0.5, 0.5, 0.5)),
                    )).insert(Node {
                        position_type: PositionType::Absolute,
                        top: px(2.0),
                        left: px(4.0),
                        ..default()
                    });
                    
                    // Skill text label inside
                    slot.spawn((
                        Text::new(""),
                        TextFont { font_size: 14.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.9, 1.0)),
                        TextLayout::new_with_justify(Justify::Center),
                    ));
                });
            }
        });
    });
}

fn handle_hud_buttons(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BagButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::StashMenu);
        }
    }
}

fn update_hud_stats(
    player_query: Query<(&Health, &PlayerStats), With<Player>>,
    equipment: Res<crate::components::PlayerEquipment>,
    mut hp_bar_query: Query<&mut Node, With<HealthBarFill>>,
    mut hp_text_query: Query<&mut Text, (With<HealthText>, Without<SkillsUI>)>,
    mut skills_query: Query<&Children, With<SkillsUI>>,
    mut slots_query: Query<&Children>,
    mut text_query: Query<&mut Text, (Without<HealthText>, Without<SkillsUI>)>,
) {
    if let Some((health, _stats)) = player_query.iter().next() {
        if let Some(mut node) = hp_bar_query.iter_mut().next() {
            let pct = (health.current as f32 / health.max as f32).max(0.0) * 100.0;
            node.width = percent(pct);
        }
        if let Some(mut text) = hp_text_query.iter_mut().next() {
            text.0 = format!("HP: {}/{}", health.current, health.max);
        }

        if let Some(children) = skills_query.iter().next() {
            for (i, child) in children.iter().enumerate() {
                let skill_name = match i {
                    0 => equipment.skill_1.as_ref(),
                    1 => equipment.skill_2.as_ref(),
                    2 => equipment.skill_3.as_ref(),
                    3 => equipment.skill_4.as_ref(),
                    _ => None,
                }.and_then(|item| item.granted_skill.map(|s| match s {
                    crate::components::PlayerSkill::SpinBlades => "Spin",
                    crate::components::PlayerSkill::HolyBurst => "Burst",
                    crate::components::PlayerSkill::Fireball => "Fire",
                    crate::components::PlayerSkill::Dash => "Dash",
                    crate::components::PlayerSkill::Earthquake => "Quake",
                    _ => "Skill"
                })).unwrap_or("");

                // child is the node for the box, we want the inner text
                if let Ok(slot_children) = slots_query.get(child) {
                    for inner_child in slot_children.iter() {
                        if let Ok(mut text) = text_query.get_mut(inner_child) {
                            if text.0 != "1" && text.0 != "2" && text.0 != "3" && text.0 != "4" {
                                text.0 = skill_name.to_string();
                            }
                        }
                    }
                }
            }
        }
    }
}

fn cleanup_hud(mut commands: Commands, q: Query<Entity, With<HudUI>>) {
    for entity in q.iter() {
        commands.entity(entity).despawn();
    }
}
