
use bevy::prelude::*;
use bevy_fmod::fmod_studio::FmodStudio;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;

use crate::{main, AppState};

use self::{assets::MyAssets, heatstroke::track_heatstroke, audio::{FmodEvent, setup_fmod_callbacks}, window::create_musician_window};

pub mod assets;
mod heatstroke;
mod mouse;
mod window;
mod audio;
//mod pedestal;

#[derive(Component)]
pub struct InGame;

pub struct MyGamePlugin;
impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            OnEnter(AppState::Game),
            (set_scene, mouse::mouse_setup, setup_fmod_callbacks),
        )
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

        // UI
        app.add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin);
        app.add_systems(Update, create_musician_window);

        // FMOD
        app.add_event::<FmodEvent>();
        app.add_systems(Update, setup_fmod_callbacks);

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

    // #[extern C]

    // FMOD callbacks

    // let mycallback: Option<unsafe extern "C" fn(u32, *mut libfmod::ffi::FMOD_STUDIO_EVENTINSTANCE, *mut std::ffi::c_void) -> i32> = Some(|event_type, event, other|{

    // });

    // // let mycallback = Option<|num, fmod_event, test|{
    // //     printf!("test");
    // // }>;

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
        }),
    ));
}
