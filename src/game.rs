use bevy::prelude::*;

use crate::AppState;

use self::{assets::MyAssets, heatstroke::track_heatstroke};

pub mod assets;
mod heatstroke;
mod mouse;

#[derive(Component)]
pub struct InGame;

pub struct MyGamePlugin;
impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::Game), (set_scene, mouse::mouse_setup))
            .add_systems(Update, track_heatstroke.run_if(in_state(AppState::Game)))
            .add_systems(
                OnExit(AppState::Game),
                crate::utility::despawn_screen::<InGame>,
            );

        // deal with menu camera
        app.add_systems(
            OnEnter(AppState::Game),
            |mut q: Query<&mut Camera, With<crate::menu::MenuCamera>>| {
                let mut camera = q.single_mut();
                camera.is_active = false;
            },
        );
        app.add_systems(
            OnExit(AppState::Game),
            |mut q: Query<&mut Camera, With<crate::menu::MenuCamera>>| {
                let mut camera = q.single_mut();
                camera.is_active = true;
            },
        );
    }

    fn is_unique(&self) -> bool {
        true
    }
}

fn set_scene(mut commands: Commands) {
    // commands.spawn((InGame, Camera3dBundle { ..default() }));

    // spawn garage
    // commands.spawn((
    //     Name::new("Garage"),
    //     // PbrBundle {
    //     //     mesh: assets.garage_handle.clone(),
    //     //     ..default()
    //     // }
    //     // Mesh {
    //     //     scene: assets.garage_handle.clone(),
    //     //     ..default()
    //     // },
    // ));
}
