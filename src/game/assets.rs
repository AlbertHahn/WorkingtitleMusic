use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub(crate) struct MyAssets {
    #[asset(path = "house.gltf")]
    garage_handle: Handle<Mesh>,
}