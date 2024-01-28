
use bevy::prelude::*;
use bevy_fmod::fmod_studio::FmodStudio;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_mod_picking::prelude::*;

use crate::{main, AppState};

use self::{assets::MyAssets, heatstroke::{track_heatstroke, trigger_heatstroke, Heatstroke, watch_heatstroke, HeastrokeResource}, audio::{FmodEvent, setup_fmod_callbacks}, window::create_musician_window};

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
        .add_event::<Heatstroke>()
        .init_resource::<HeastrokeResource>()
        .add_systems(Update, (track_heatstroke, trigger_heatstroke, watch_heatstroke))
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


fn set_scene(mut commands: Commands, assets: Res<MyAssets>, studio: Res<FmodStudio>, mut heatstroke_resource: ResMut<HeastrokeResource>) {
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

    // Spawn pedestals
    let pedestals = vec![
        ("pedestal1".to_string(), Vec3::new(3.2, -1.6, -40.5)),
        ("pedestal2".to_string(), Vec3::new(-7.9, -0.8, -48.8)),
        ("pedestal3".to_string(), Vec3::new(-5.2, -1.7, -40.8)),
        ("pedestal4".to_string(), Vec3::new(8.2, -1.7, -48.8)),
    ];

    // pbr_bundle_spawner(&mut commands, assets.pedestal_handle.clone() , &pedestals);

    let entities = pbr_bundle_spawner(&mut commands, assets.pedestal_handle.clone() , &pedestals);

    heatstroke_resource.into_inner().pedestal_id = *entities.get(1).expect("pedestal selection failed");
    
    debug!("set pedestal");
    
    let guitar = vec![
        ("guitar".to_string(), Vec3::new(3.2, -1.6, -40.5)),
    ];
    let drums = vec![
        ("drums".to_string(), Vec3::new(-7.9, -0.8, -48.8)),
    ];
    let streicher = vec![
        ("streicher".to_string(), Vec3::new(-5.2, -1.7, -40.8)),
    ];

    let keyboard = vec![
        ("keyboard".to_string(), Vec3::new(8.2, -1.7, -48.8)),
    ];

    scene_spawner(&mut commands, assets.musician_guitar_scene.clone() , &guitar);
    scene_spawner(&mut commands, assets.musician_drums_scene.clone() , &drums);
    scene_spawner(&mut commands, assets.musician_streicher_scene.clone() , &streicher);
    scene_spawner(&mut commands, assets.musician_keyboard_scene.clone() , &keyboard);

}


fn pbr_bundle_spawner(
    mut commands : &mut Commands,
    mesh_handle: Handle<Mesh>,
    properties: &[(String, Vec3)],
) -> std::vec::Vec<bevy::prelude::Entity> {
    let mut entities = Vec::<Entity>::new();
    for (name, coordinates) in properties {
        let e = commands.spawn((
            Name::new(name.clone()),
            PbrBundle {
                mesh: mesh_handle.clone(),
                transform: Transform::from_translation(*coordinates),
                ..Default::default()
            },
            PickableBundle::default(), // <- Makes the mesh pickable.
            On::<Pointer<Click>>::target_commands_mut(|_click, target_commands| {
                target_commands.despawn();
            }),
        ));
        entities.push(e.id());
    }
    return entities
}


fn scene_spawner(
    mut commands : &mut Commands,
    scene_handle: Handle<Scene>,
    properties: &[(String, Vec3)],
) {
    for (name, coordinates) in properties {
        commands.spawn((
            Name::new(name.clone()),
            SceneBundle {
                scene: scene_handle.clone(),
                transform: Transform::from_translation(*coordinates),
                ..default()
            },
            PickableBundle::default(), // <- Makes the mesh pickable.
            On::<Pointer<Click>>::target_commands_mut(|_click, target_commands| {
                target_commands.despawn();
            }),
        ));
    }
}