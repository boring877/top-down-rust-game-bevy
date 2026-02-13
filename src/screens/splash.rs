use bevy::prelude::*;
use crate::game_state::GameState;


#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);


pub fn splash_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)));
}

fn splash_setup(mut commands: Commands) {
    //spawn the camera !

    commands.spawn(Camera2d);

    // spawn UI with title

    commands.spawn((

        //node for the UI
        Node {
            width: percent (100),
            height: percent (100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children! [(
            Text::new("Player VS Boss"),
            TextFont {
                font_size: 80.0,
                ..default()
            },
            TextColor(Color::WHITE),
            
        )
            
        ]

    ));

    // Timer for 2 seconds
    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)))
} 

fn countdown(
    time:Res<Time>, // time global shard resource !!
    mut timer:ResMut<SplashTimer>, // timer write resource !!
    mut next_state: ResMut<NextState<GameState>>, //change game state
) {
    if timer.tick(time.delta()).just_finished() {
        next_state.set(GameState::NameEntry);
    }
}