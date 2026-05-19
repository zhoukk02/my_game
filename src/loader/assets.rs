use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::data::{AudioData, ItemData};

#[derive(Resource, AssetCollection)]
pub struct GameAsset {
    #[asset(path = "data/items", collection(typed))]
    pub items: Vec<Handle<ItemData>>,
    #[asset(path = "data/audio", collection(typed))]
    pub audio: Vec<Handle<AudioData>>,
}
