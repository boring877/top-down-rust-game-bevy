use crate::components::DamageNumber;
use crate::constants::*;
use bevy::prelude::*;

pub fn spawn_damage_number(commands: &mut Commands, damage: u32, position: Vec2) {
    commands.spawn((
        DamageNumber {
            timer: Timer::from_seconds(DAMAGE_NUMBER_DURATION, TimerMode::Once),
            damage,
        },
        Text2d::new(format!("{}", damage)),
        TextFont {
            font_size: DAMAGE_NUMBER_FONT_SIZE,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.8, 0.2)),
        Transform::from_xyz(position.x, position.y + DAMAGE_NUMBER_OFFSET_Y, 3.0),
    ));
}

pub fn animate_damage_numbers(
    time: Res<Time>,
    mut commands: Commands,
    mut damage_numbers: Query<(Entity, &mut DamageNumber, &mut Transform, &mut TextColor)>,
) {
    for (entity, mut damage_number, mut transform, mut color) in damage_numbers.iter_mut() {
        damage_number.timer.tick(time.delta());

        transform.translation.y += DAMAGE_NUMBER_SPEED * time.delta_secs();

        let progress = damage_number.timer.fraction();
        let alpha = 1.0 - progress;
        color.0 = color.0.with_alpha(alpha);

        if damage_number.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
