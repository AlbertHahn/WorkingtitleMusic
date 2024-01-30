use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(path = "house_v2.gltf#Scene0")]
    pub garage_handle: Handle<Scene>,


    // #[asset(path = "garage musician guitar.gltf#Scene0")]
    // pub musician_guitar_scene: Handle<Scene>,
    
    // Drummer
    #[asset(path = "garage musician drums.gltf#Scene0")]
    pub musician_drums_scene: Handle<Scene>,
    // #[asset(path = "garage musician drums.gltf#Mesh0")]
    // pub musician_drums_mesh: Handle<Mesh>,
    #[asset(path = "garage musician drums.gltf#Animation0")]
    pub musician_drums_animation: Handle<AnimationClip>,

    // Streicher
    #[asset(path = "garage musician streicher.gltf#Scene0")]
    pub musician_streicher_scene: Handle<Scene>,
    #[asset(path = "garage musician streicher.gltf#Animation0")]
    pub musician_streicher_anim: Handle<AnimationClip>,

    // keyboard
    #[asset(path = "garage musician keyboard.gltf#Scene0")]
    pub musician_keyboard_scene: Handle<Scene>,
    #[asset(path = "garage musician keyboard.gltf#Animation0")]
    pub musician_keyboard_anim: Handle<AnimationClip>,

    // guitar
    #[asset(path = "garage musician guitar.gltf#Scene0")]
    pub musician_guitar_scene: Handle<Scene>,
    #[asset(path = "garage musician guitar.gltf#Animation0")]
    pub musician_guitar_anim: Handle<AnimationClip>,

    #[asset(path = "garage musician model.gltf#Scene0")]
    pub musician_model_scene: Handle<Scene>,

    #[asset(path = "pedestal.gltf#Mesh0")]
    pub pedestal_handle: Handle<Mesh>,

    #[asset(path ="textures/facialexpressions/maleeyes1/", collection(typed))]
    pub eyes_healthy: Vec<Handle<Image>>,
    #[asset(path ="textures/facialexpressions/hurt1male", collection(typed))]
    pub eyes_damaged: Vec<Handle<Image>>,
    #[asset(path ="textures/facialexpressions/dedeyes", collection(typed))]
    pub eyes_dead: Vec<Handle<Image>>,

    // Menu UI
    #[asset(path ="textures/ui/titlescreen.png")]
    pub menu_background: Handle<Image>,
    #[asset(path ="textures/ui/titlescreenStart_cropped.png")]
    pub menu_play: Handle<Image>,
    #[asset(path ="textures/ui/titlescreenQuit_cropped.png")]
    pub menu_quit: Handle<Image>,
    
}