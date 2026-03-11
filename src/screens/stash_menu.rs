use crate::components::*;
use crate::game_state::GameState;
use bevy::prelude::*;

#[derive(Event)]
struct RefreshStashMenuEvent;

pub fn stash_menu_plugin(app: &mut App) {
    app.add_observer(refresh_stash_menu)
        .add_systems(OnEnter(GameState::StashMenu), (setup_stash_menu, pause_game))
        .add_systems(OnExit(GameState::StashMenu), (cleanup_stash_menu, resume_game))
        .add_systems(
            Update,
            (
                handle_stash_buttons,
                handle_stash_input,
                handle_equipment_interactions,
                update_item_tooltip,
            ).run_if(in_state(GameState::StashMenu)),
        )
        .add_systems(
            Update,
            handle_inventory_key.run_if(in_state(GameState::Game)),
        );
}

fn pause_game(mut time: ResMut<Time<Virtual>>) {
    time.pause();
}

fn resume_game(mut time: ResMut<Time<Virtual>>) {
    time.unpause();
}

#[derive(Component)]
struct StashMenuUI;

#[derive(Component)]
struct CloseStashButton;

#[derive(Component)]
struct EquipSlotButton(PickupType, usize);

#[derive(Component)]
struct InventoryItemButton(usize);

#[derive(Component)]
struct ItemTooltipPanel;

#[derive(Component)]
struct ItemTooltipName;

#[derive(Component)]
struct ItemTooltipStats;

#[derive(Component)]
struct HoverableItem(crate::components::Item);

macro_rules! spawn_slot {
    ($builder:expr, $label:expr, $item:expr, $slot_type:expr, $idx:expr) => {
        {
            let has_item = $item.is_some();
            let text = if has_item { $item.as_ref().unwrap().name.clone() } else { "Empty".to_string() };
            let text_color = if has_item { $item.as_ref().unwrap().rarity.color() } else { Color::srgb(0.4, 0.3, 0.3) };
            
            // Diablo style earthy/dark colors
            // Dark brown/gray background, rusty gold border if full, dark red if empty
            let bg_color = if has_item { Color::srgb(0.12, 0.08, 0.05) } else { Color::srgb(0.05, 0.04, 0.04) };
            
            let mut entity = $builder.spawn((
                Button,
                EquipSlotButton($slot_type, $idx),
                Node {
                    width: px(100.0),
                    height: px(100.0),
                    margin: UiRect::all(px(5.0)),
                    padding: UiRect::all(px(8.0)),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                BackgroundColor(bg_color),
            ));
            
            if has_item {
                entity.insert(HoverableItem($item.as_ref().unwrap().clone()));
            }

            entity.with_children(|parent| {
                parent.spawn((
                    Text::new($label),
                    TextFont { font_size: 16.0, ..default() },
                    TextColor(Color::srgb(0.6, 0.5, 0.4)),
                ));
                
                // Item Name
                parent.spawn((
                     Text::new(text),
                     TextFont { font_size: 14.0, ..default() },
                     TextColor(text_color),
                ));
            });
        }
    };
}

macro_rules! spawn_bag_item {
    ($builder:expr, $item:expr, $index:expr) => {
        $builder.spawn((
            Button,
            InventoryItemButton($index),
            HoverableItem($item.clone()),
            Node {
                width: px(90.0),
                height: px(90.0),
                margin: UiRect::all(px(5.0)),
                padding: UiRect::all(px(5.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.1, 0.08)),
        )).with_children(|parent| {
            parent.spawn((
                Text::new($item.name.clone()),
                TextFont { font_size: 14.0, ..default() },
                TextColor($item.rarity.color()),
                TextLayout::new_with_justify(Justify::Center), 
            ));
        });
    };
}

fn cleanup_stash_menu(mut commands: Commands, query: Query<Entity, With<StashMenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn setup_stash_menu(
    mut commands: Commands,
    player_query: Query<(&PlayerStats, &CombatStats, &Player), With<Player>>,
    equipment: Res<PlayerEquipment>,
) {
    spawn_stash_ui(&mut commands, &player_query, &equipment);
}

fn refresh_stash_menu(
    _trigger: On<RefreshStashMenuEvent>,
    mut commands: Commands,
    ui_query: Query<Entity, With<StashMenuUI>>,
    player_query: Query<(&PlayerStats, &CombatStats, &Player), With<Player>>,
    equipment: Res<PlayerEquipment>,
) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn();
    }
    spawn_stash_ui(&mut commands, &player_query, &equipment);
}

fn spawn_stash_ui(
    commands: &mut Commands,
    player_query: &Query<(&PlayerStats, &CombatStats, &Player), With<Player>>,
    equipment: &Res<PlayerEquipment>,
) {
    let mut level = 1;
    let mut xp = 0;
    let mut max_xp = 100;
    let mut gold = 0;
    let mut materials = 0;

    let mut intelligence = 10;
    let mut strength = 10;
    let mut agility = 10;
    let mut dodge_rate = 0.05;
    let mut crit_rate = 0.05;
    let mut crit_damage = 1.5;
    let mut p_class = crate::components::PlayerClass::Warrior;

    if let Some((stats, combat, _player)) = player_query.iter().next() {
        level = stats.level;
        xp = stats.xp;
        max_xp = stats.max_xp;
        gold = stats.gold;
        materials = stats.materials;
        p_class = equipment.player_class;

        let total = equipment.get_total_stats(combat);
        intelligence = total.intelligence;
        strength = total.strength;
        agility = total.agility;
        dodge_rate = total.dodge_rate;
        crit_rate = total.crit_rate;
        crit_damage = total.crit_damage;
    }

    // Outer dark overlay
    commands.spawn((
        StashMenuUI,
        Node {
            width: percent(100.0),
            height: percent(100.0),
            position_type: PositionType::Absolute,
            left: px(0.0),
            top: px(0.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.01, 0.01, 0.92)), // Deep dark red/brown
    )).with_children(|parent| {
        // Main gothic panel
        parent.spawn((
            Node {
                width: px(1100.0),
                height: px(750.0),
                padding: UiRect::all(px(30.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.06, 0.04, 0.03)), // Gothic dark bg
        )).with_children(|menu| {
            // TITLE
            menu.spawn((
                Text::new("CHARACTER STATUS"),
                TextFont { font_size: 45.0, ..default() },
                TextColor(Color::srgb(0.7, 0.1, 0.1)), // Blood red
            ));

            // Content Split (Left: Stats/Equip, Right: Bag)
            menu.spawn(Node {
                width: percent(100.0),
                height: percent(100.0),
                flex_direction: FlexDirection::Row,
                margin: UiRect::top(px(20.0)),
                ..default()
            }).with_children(|content| {
                
                // LEFT COLUMN: Equipment & Stats
                content.spawn(Node {
                    width: percent(50.0),
                    height: percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::right(px(15.0)),
                    ..default()
                }).with_children(|left_col| {
                    
                    // Stats Box
                    left_col.spawn(Node {
                        width: percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(px(15.0)),
                        padding: UiRect::all(px(15.0)),
                        ..default()
                    }).insert(BackgroundColor(Color::srgb(0.08, 0.06, 0.05)))
                    .with_children(|stats_box| {
                        stats_box.spawn((
                            Text::new(format!("{:?} - Level {} (XP: {}/{})", p_class, level, xp, max_xp)),
                            TextFont { font_size: 22.0, ..default() },
                            TextColor(Color::srgb(0.9, 0.8, 0.6)), // Old gold
                        ));
                        stats_box.spawn((
                            Text::new(format!("Gold: {}  |  Materials: {}", gold, materials)),
                            TextFont { font_size: 20.0, ..default() },
                            TextColor(Color::srgb(0.8, 0.6, 0.2)),
                        ));
                        
                        // Combat Stats
                        stats_box.spawn(Node {
                            margin: UiRect::top(px(15.0)),
                            width: percent(100.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            row_gap: px(5.0),
                            ..default()
                        }).with_children(|c_stats| {
                            c_stats.spawn((Text::new(format!("STR (Max HP): {}", strength)), TextFont { font_size: 18.0, ..default() }, TextColor(Color::srgb(0.9, 0.4, 0.4))));
                            c_stats.spawn((Text::new(format!("INT (Magic Atk): {}", intelligence)), TextFont { font_size: 18.0, ..default() }, TextColor(Color::srgb(0.5, 0.6, 0.9))));
                            c_stats.spawn((Text::new(format!("AGI (Phys Atk): {}", agility)), TextFont { font_size: 18.0, ..default() }, TextColor(Color::srgb(0.5, 0.9, 0.5))));
                            c_stats.spawn((Text::new(format!("CRIT RATE: {:.1}%", crit_rate * 100.0)), TextFont { font_size: 18.0, ..default() }, TextColor(Color::srgb(0.9, 0.9, 0.2))));
                            c_stats.spawn((Text::new(format!("CRIT DMG: {:.1}%", crit_damage * 100.0)), TextFont { font_size: 18.0, ..default() }, TextColor(Color::srgb(0.9, 0.6, 0.2))));
                            c_stats.spawn((Text::new(format!("DODGE: {:.1}%", dodge_rate * 100.0)), TextFont { font_size: 18.0, ..default() }, TextColor(Color::srgb(0.8, 0.6, 0.8))));
                        });
                    });

                    // Equipment Grid
                    left_col.spawn(Node {
                        width: percent(100.0),
                        justify_content: JustifyContent::Center,
                        flex_wrap: FlexWrap::Wrap,
                        row_gap: px(10.0),
                        column_gap: px(10.0),
                        margin: UiRect::bottom(px(15.0)),
                        ..default()
                    }).with_children(|grid| {
                        spawn_slot!(grid, "Weapon", &equipment.weapon, PickupType::Weapon, 0);
                        spawn_slot!(grid, "Helmet", &equipment.helmet, PickupType::Helmet, 0);
                        spawn_slot!(grid, "Armor", &equipment.armor, PickupType::Armor, 0);
                        spawn_slot!(grid, "Pants", &equipment.pants, PickupType::Pants, 0);
                        spawn_slot!(grid, "Shoes", &equipment.shoes, PickupType::Shoes, 0);
                        spawn_slot!(grid, "Ring", &equipment.ring, PickupType::Ring, 0);
                        spawn_slot!(grid, "Earring", &equipment.earring, PickupType::Earring, 0);
                        spawn_slot!(grid, "Necklace", &equipment.necklace, PickupType::Necklace, 0);
                        spawn_slot!(grid, "Gemstone", &equipment.gemstone, PickupType::Gemstone, 0);
                    });

                    // Skill Grid
                    left_col.spawn(Node {
                        width: percent(100.0),
                        justify_content: JustifyContent::Center,
                        flex_wrap: FlexWrap::Wrap,
                        row_gap: px(10.0),
                        column_gap: px(10.0),
                        ..default()
                    }).with_children(|grid| {
                        spawn_slot!(grid, "Skill 1", &equipment.skill_1, PickupType::SkillGem, 1);
                        spawn_slot!(grid, "Skill 2", &equipment.skill_2, PickupType::SkillGem, 2);
                        spawn_slot!(grid, "Skill 3", &equipment.skill_3, PickupType::SkillGem, 3);
                        spawn_slot!(grid, "Skill 4", &equipment.skill_4, PickupType::SkillGem, 4);
                    });
                });

                // RIGHT COLUMN: Inventory Bag
                content.spawn(Node {
                    width: percent(50.0),
                    height: percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    padding: UiRect::left(px(15.0)),
                    ..default() 
                }).with_children(|right_col| {
                    right_col.spawn((
                        Text::new("INVENTORY"),
                        TextFont { font_size: 30.0, ..default() },
                        TextColor(Color::srgb(0.7, 0.5, 0.3)),
                    ));

                    right_col.spawn((
                        Node {
                            width: percent(100.0),
                            height: px(500.0),
                            margin: UiRect::top(px(15.0)),
                            padding: UiRect::all(px(10.0)),
                            flex_wrap: FlexWrap::Wrap,
                            align_items: AlignItems::FlexStart,
                            align_content: AlignContent::FlexStart,
                            overflow: Overflow::clip_y(),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.04, 0.03, 0.02)), // Very dark bag
                    )).with_children(|bag| {
                        if equipment.inventory.is_empty() {
                            bag.spawn((
                                Text::new("Your bags are empty..."),
                                TextFont { font_size: 18.0, ..default() },
                                TextColor(Color::srgb(0.4, 0.3, 0.3)),
                            ));
                        } else {
                            for (i, item) in equipment.inventory.iter().enumerate() {
                                spawn_bag_item!(bag, item, i);
                            }
                        }
                    });

                    // CLOSE BUTTON
                    right_col.spawn((
                        Button,
                        CloseStashButton,
                        Node {
                            margin: UiRect::top(px(30.0)),
                            width: px(250.0),
                            height: px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.4, 0.1, 0.1)),
                    )).with_children(|btn| {
                        btn.spawn((
                            Text::new("RETURN [ESC]"),
                            TextFont { font_size: 20.0, ..default() },
                            TextColor(Color::srgb(0.9, 0.8, 0.7)),
                        ));
                    });

                });
            });
        });

        // Tooltip panel
        parent.spawn((
            StashMenuUI,
            ItemTooltipPanel,
            Node {
                position_type: PositionType::Absolute,
                display: Display::None,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(px(15.0)),
                border: UiRect::all(px(0.0)),
                row_gap: px(5.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.06, 0.05, 0.95)),
        )).with_children(|tt_parent| {
            tt_parent.spawn((
                ItemTooltipName,
                Text::new(""),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::WHITE),
            ));
            tt_parent.spawn((
                ItemTooltipStats,
                Text::new(""),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
        });
    });
}

fn handle_equipment_interactions(
    mut interaction_query: Query<
        (&Interaction, Option<&EquipSlotButton>, Option<&InventoryItemButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut equipment: ResMut<PlayerEquipment>,
    mut commands: Commands,
) {
    let mut changed = false;

    for (interaction, slot_btn, inv_btn) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // Unequip item clicked inside a slot
            if let Some(slot) = slot_btn {
                let unequipped = match slot.0 {
                    PickupType::Weapon => equipment.weapon.take(),
                    PickupType::Helmet => equipment.helmet.take(),
                    PickupType::Armor => equipment.armor.take(),
                    PickupType::Pants => equipment.pants.take(),
                    PickupType::Shoes => equipment.shoes.take(),
                    PickupType::Ring => equipment.ring.take(),
                    PickupType::Earring => equipment.earring.take(),
                    PickupType::Necklace => equipment.necklace.take(),
                    PickupType::Gemstone => equipment.gemstone.take(),
                    PickupType::SkillGem => {
                        match slot.1 {
                            1 => equipment.skill_1.take(),
                            2 => equipment.skill_2.take(),
                            3 => equipment.skill_3.take(),
                            4 => equipment.skill_4.take(),
                            _ => None,
                        }
                    }
                    _ => None,
                };
                if let Some(item) = unequipped {
                    equipment.inventory.push(item);
                    changed = true;
                }
            }
            
            // Equip item clicked from inventory bag
            if let Some(inv) = inv_btn {
                let index = inv.0;
                if index < equipment.inventory.len() {
                    let item = equipment.inventory.remove(index);
                    let previous_equip = match item.pickup_type {
                        PickupType::Weapon => equipment.weapon.replace(item),
                        PickupType::Helmet => equipment.helmet.replace(item),
                        PickupType::Armor => equipment.armor.replace(item),
                        PickupType::Pants => equipment.pants.replace(item),
                        PickupType::Shoes => equipment.shoes.replace(item),
                        PickupType::Ring => equipment.ring.replace(item),
                        PickupType::Earring => equipment.earring.replace(item),
                        PickupType::Necklace => equipment.necklace.replace(item),
                        PickupType::Gemstone => equipment.gemstone.replace(item),
                        PickupType::SkillGem => {
                            if equipment.skill_1.is_none() { equipment.skill_1.replace(item) }
                            else if equipment.skill_2.is_none() { equipment.skill_2.replace(item) }
                            else if equipment.skill_3.is_none() { equipment.skill_3.replace(item) }
                            else if equipment.skill_4.is_none() { equipment.skill_4.replace(item) }
                            else { equipment.skill_1.replace(item) } // Overwrite 1 if full
                        }
                        _ => {
                            // If it's not an equipment, put it back
                            equipment.inventory.insert(index, item);
                            None
                        }
                    };
                    
                    if let Some(prev) = previous_equip {
                        equipment.inventory.push(prev);
                    }
                    changed = true;
                }
            }
        }
    }

    if changed {
        commands.trigger(RefreshStashMenuEvent);
    }
}

fn handle_stash_buttons(
    close_query: Query<&Interaction, With<CloseStashButton>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in close_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Game);
        }
    }
}

fn handle_stash_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Tab) || keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::KeyI) {
        next_state.set(GameState::Game);
    }
}

fn handle_inventory_key(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Tab) || keyboard.just_pressed(KeyCode::KeyI) {
        next_state.set(GameState::StashMenu);
    }
}

fn update_item_tooltip(
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    q_hoverable: Query<(&Interaction, &HoverableItem)>,
    mut q_tooltip: Query<&mut Node, With<ItemTooltipPanel>>,
    mut q_name: Query<&mut Text, (With<ItemTooltipName>, Without<ItemTooltipStats>)>,
    mut q_stats: Query<&mut Text, (With<ItemTooltipStats>, Without<ItemTooltipName>)>,
    mut q_name_color: Query<&mut TextColor, With<ItemTooltipName>>,
) {
    let mut hovered_item = None;
    for (interaction, item) in q_hoverable.iter() {
        if *interaction == Interaction::Hovered {
            hovered_item = Some(&item.0);
            break;
        }
    }

    if let Some(mut style) = q_tooltip.iter_mut().next() {
        if let Some(item) = hovered_item {
            style.display = Display::Flex;
            
            if let Some(window) = q_windows.iter().next() {
                if let Some(pos) = window.cursor_position() {
                    // Offset to avoid hiding behind cursor
                    style.left = Val::Px(pos.x as f32 + 15.0);
                    style.top = Val::Px(pos.y as f32 + 15.0);
                }
            }

            if let Some(mut text) = q_name.iter_mut().next() {
                text.0 = item.name.clone();
            }
            if let Some(mut color) = q_name_color.iter_mut().next() {
                color.0 = item.rarity.color();
            }
            if let Some(mut text) = q_stats.iter_mut().next() {
                let skill_text = if let Some(skill) = &item.granted_skill {
                    format!("Skill: {:?}\n*{:?}*\n{}\n\n", skill, skill.description(), skill.damage_info())
                } else {
                    String::new()
                };

                text.0 = format!(
                    "{}STR: +{}\nINT: +{}\nAGI: +{}\nCRIT: {:.1}%\nCRIT DMG: {:.1}%\nDODGE: {:.1}%",
                    skill_text, item.strength, item.intelligence, item.agility,
                    item.crit_rate * 100.0, item.crit_damage * 100.0, item.dodge_rate * 100.0
                );
            }
        } else {
            style.display = Display::None;
        }
    }
}
