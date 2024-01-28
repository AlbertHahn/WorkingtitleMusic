use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(path = "house_v2.gltf#Scene0")]
    pub garage_handle: Handle<Scene>,
    #[asset(path = "garage musician guitar.gltf#Scene0")]
    pub musician_guitar_scene: Handle<Scene>,
    #[asset(path = "garage musician drums.gltf#Scene0")]
    pub musician_drums_scene: Handle<Scene>,
    #[asset(path = "garage musician streicher.gltf#Scene0")]
    pub musician_streicher_scene: Handle<Scene>,
    #[asset(path = "garage musician keyboard.gltf#Scene0")]
    pub musician_keyboard_scene: Handle<Scene>,
    #[asset(path = "pedestal.gltf#Mesh0")]
    pub pedestal_handle: Handle<Mesh>,
    #[asset(path ="textures/facialexpressions/maleeyes1/", collection(typed))]
    pub eyes_healthy: Vec<Handle<Image>>,
    #[asset(path ="textures/facialexpressions/hurt1male", collection(typed))]
    pub eyes_damaged: Vec<Handle<Image>>,
    #[asset(path ="textures/facialexpressions/dedeyes", collection(typed))]
    pub eyes_dead: Vec<Handle<Image>>,
}