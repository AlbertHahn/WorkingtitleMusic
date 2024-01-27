use bevy::prelude::*;

use crate::AppState;

struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), systems);
    }
}

fn setup_menu(){
    
}