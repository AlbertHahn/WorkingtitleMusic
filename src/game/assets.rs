use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    // #[asset(path = "house.gltf")]
    // pub garage_handle: Handle<Mesh>,
    #[asset(path = "garage musician default.gltf")]
    pub musician_default: Handle<Mesh>,
    #[asset(paths("textures/facialexpressions/maleeyes1"), collection(typed))]
    pub eyes_healthy: Vec<Handle<Image>>,
    #[asset(paths("textures/facialexpressions/hurt1male"), collection(typed))]
    pub eyes_damaged: Vec<Handle<Image>>,
    #[asset(paths("textures/facialexpressions/dedeyes"), collection(typed))]
    pub eyes_dead: Vec<Handle<Image>>,
}