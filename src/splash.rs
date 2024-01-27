use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{AppState, utility, game::assets::MyAssets};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            .add_state::<AssetLoadingState>()
            .add_loading_state(
                LoadingState::new(AssetLoadingState::Loading)
                    .continue_to_state(AssetLoadingState::Finished)
                    .load_collection::<MyAssets>(),
            )
            // When entering the state, spawn everything needed for this screen
            .add_systems(Startup, splash_setup)
            // .add_systems(OnEnter(AppState::Splash), splash_setup)
            // While in this state, run the `countdown` system
            // .add_systems(Update, countdown.run_if(in_state(AppState::Splash)))
            // When exiting the state, despawn everything that was spawned for this screen
            // .add_systems(Update, goto_menu.run_if(||{

            // }))
            .add_systems(OnEnter(AssetLoadingState::Finished), goto_menu)
            .add_systems(OnExit(AppState::Splash), utility::despawn_screen::<OnSplashScreen>);
    }
}

#[derive(Component)]
struct OnSplashScreen;

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("deploy splash screen!");

    let fmod_icon = asset_server.load("FMOD Logo White - Black Background.png");
    // Display the logo
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    // This will set the logo to be 200px wide, and auto adjust its height
                    width: Val::Px(400.0),
                    ..default()
                },
                image: UiImage::new(fmod_icon),
                ..default()
            });
        });
    // Insert the timer as a resource
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Reflect)]
enum AssetLoadingState{
    #[default]
    Loading,
    Finished
}

fn countdown(
    mut game_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(AppState::Menu);
    }
}

fn goto_menu(mut state: ResMut<State<AppState>>){
    // let _ = state.set(AppState::Menu).expect("error transitioning to menu");
    debug!("survived state transition somehow");
}