use bevy::prelude::*;
use bevy_fmod::fmod_plugin::FmodPlugin;

mod game;
mod splash;
mod utility;
mod menu;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
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

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FmodPlugin {
            audio_banks_paths: &FMOD_BANKS,
        })
        .add_plugins(game::MyGamePlugin)
        .add_state::<AppState>()
        // .add_systems(, systems)
        .run();
}

fn load_assets(commands: Commands){
    
}

