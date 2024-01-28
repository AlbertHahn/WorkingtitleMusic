use std::ffi::c_void;

use bevy::prelude::*;
use bevy_fmod::fmod_studio::FmodStudio;
use libfmod::ffi::{FMOD_STUDIO_EVENT_CALLBACK_TYPE, FMOD_STUDIO_EVENTINSTANCE, FMOD_RESULT};
// use libfmod::ffi;

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


enum LevelEvent {
    TruiggerHeatstroke

}
#[derive(Event)]
struct LevelEndEvent;

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

    #[no_mangle]
    unsafe extern "C" fn heatstroke_callback(type_: FMOD_STUDIO_EVENT_CALLBACK_TYPE, event: *mut FMOD_STUDIO_EVENTINSTANCE, parameters: *mut c_void) -> FMOD_RESULT{
        debug!("Hello from fmod!");
        debug!("{:?}   {:?}", type_, event);
        info!("hmmm... heatstroke?");
        return libfmod::ffi::FMOD_OK;
    }
    let _ = studio.0.get_event("event:/triggers/trigger_heatstroke").unwrap().set_callback( Some(heatstroke_callback), 0x0000_0004u32);



    #[no_mangle]
    unsafe extern "C" fn level_end_callback(type_: FMOD_STUDIO_EVENT_CALLBACK_TYPE, event: *mut FMOD_STUDIO_EVENTINSTANCE, parameters: *mut c_void) -> FMOD_RESULT{
        debug!("Hello from fmod!");
        debug!("{:?}   {:?}", type_, event);
        info!("level complete!");
        return libfmod::ffi::FMOD_OK;
    }
    let _ = studio.0.get_event("event:/triggers/trigger_levelend").unwrap().set_callback( Some(level_end_callback), 0x0000_0004u32);


    // let mycallback: Option<unsafe extern "C" fn(u32, *mut libfmod::ffi::FMOD_STUDIO_EVENTINSTANCE, *mut std::ffi::c_void) -> i32> = Some(|event_type, event, other|{

    // });

    // // let mycallback = Option<|num, fmod_event, test|{
    // //     printf!("test");
    // // }>;

    // main_menu_player
    //     .event_instance
    //     .set_callback( Some(mycallback), 0x00020000_u32);

    // spawn garage
    commands.spawn((
        Name::new("Garage"),
        SceneBundle {
            scene: assets.garage_handle.clone(),
            ..default()
        },
    ));
}
