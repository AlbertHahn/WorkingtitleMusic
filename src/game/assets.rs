use bevy::asset::Handle;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
struct AudioAssets {
    #[asset(path = "house.gltf")]
    garage_handle: Handle<Mesh>,
}