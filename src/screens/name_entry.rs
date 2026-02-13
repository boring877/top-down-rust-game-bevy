//This is name entry for the game Player vs Boss

use bevy::prelude::*;
use crate::game_state::GameState;


pub fn name_entry_plugin(app: &mut App) {
    app
        
        //Run once on entry !!!
        .add_systems(OnEnter(GameState::NameEntry), setup_name_entry)

        //Run every frame while in this state !!!
        .add_systems(Update, handle_name_input.run_if(in_state(GameState::NameEntry)));

}


fn setup_name_entry(mut commands: Commands) {
    
}

fn handle_name_input(mut commands: Commands) {
    
}
