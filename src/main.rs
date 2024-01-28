use bevy::prelude::*;
use bevy_fmod::fmod_plugin::FmodPlugin;
use bevy_inspector_egui::quick::StateInspectorPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod game;
mod menu;
mod splash;
mod utility;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Reflect)]
pub enum AppState {
    #[default]
    Splash,
    Menu,
    Game,
}

fn main() {
    use const_str::concat;

    #[cfg(not(debug_assertions))]
    const FMOD_ROOT: &str = "assets/audio/";

    #[cfg(debug_assertions)]
    const FMOD_ROOT: &str = "audio/Build/Desktop/";
    const FMOD_BANKS: [&'static str; 2] = [
        concat!(FMOD_ROOT, "Master.bank"),
        concat!(FMOD_ROOT, "Master.strings.bank"),
    ];

    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(FmodPlugin {
            audio_banks_paths: &FMOD_BANKS,
        })
        .add_plugins(game::MyGamePlugin)
        .add_plugins(menu::MenuPlugin)
        .add_plugins(splash::SplashPlugin)
        .add_state::<AppState>()
        // .add_systems(, systems)
        ;
    #[cfg(debug_assertions)]
    {
        // app.register_type::<AppState>();
        // app.add_plugins(StateInspectorPlugin::<AppState>::default());
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.add_systems(Startup, |mut commands: Commands| {
        commands.spawn((
            Name::new("MenuCamera"),
            menu::MenuCamera,
            Camera2dBundle { ..default() },
        ));
    });

    app.run();
}
