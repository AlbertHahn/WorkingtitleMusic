use bevy::prelude::*;
use bevy_fmod::fmod_studio::FmodStudio;
use bevy_mod_picking::prelude::*;

use crate::{main, AppState};

use self::{assets::MyAssets, heatstroke::track_heatstroke};

pub mod assets;
mod heatstroke;
mod mouse;
//mod pedestal;

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

fn set_scene(mut commands: Commands, assets: Res<MyAssets>, studio: Res<FmodStudio>) {
    // create camera for the level
    commands.spawn((InGame, Camera3dBundle { ..default() }));

    // start level track
    let event_description = studio.0.get_event("event:/Levels/1/main").unwrap();

    let main_menu_player = bevy_fmod::prelude::AudioSource::new(event_description);
    main_menu_player.play();
    commands.spawn((InGame, main_menu_player));

    // main_menu_player
    //     .event_instance
    //     .set_callback(|| {}, callbackmask);

    // spawn garage
    commands.spawn((
        Name::new("Garage"),
        SceneBundle {
            scene: assets.garage_handle.clone(),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("pedestal"),
        PbrBundle {
            mesh: assets.pedestal_handle.clone(),
            ..default()
        },
        PickableBundle::default(), // <- Makes the mesh pickable.
        On::<Pointer<Click>>::target_commands_mut(|_click, target_commands| {    
            target_commands.despawn();
        })
    ));



}