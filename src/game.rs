use bevy::prelude::*;

use crate::AppState;

use self::heatstroke::track_heatstroke;

pub mod assets;
mod heatstroke;
mod mouse;

#[derive(Component)]
pub struct InGame;

pub struct MyGamePlugin;
impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, set_scene)
            .add_systems(OnEnter(AppState::Game), mouse::mouse_setup)
            .add_systems(Update, track_heatstroke.run_if(in_state(AppState::Game)))
            .add_systems(
                OnExit(AppState::Game),
                crate::utility::despawn_screen::<InGame>,
            );
    }

    fn is_unique(&self) -> bool {
        true
    }
}

fn set_scene(mut commands: Commands) {
    commands.spawn((InGame, Camera3dBundle { ..default() }));
}
