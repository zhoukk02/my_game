use bevy::prelude::*;

use crate::{
    app::AppState,
    data::{AudioData, ItemData, Store},
};

use super::assets::GameAsset;

pub fn game_ready(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
) {
    commands.remove_resource::<GameAsset>();
    next_state.set(AppState::Running);
}

pub fn store_item(
    mut commands: Commands,
    collection: Res<GameAsset>,
    assets: Res<Assets<ItemData>>,
) {
    let mut hm = Store::<ItemData>::new();
    for handle in collection.items.iter() {
        let Some(asset) = assets.get(handle) else {
            return;
        };
        hm.add(asset.id, asset.clone());
    }
    commands.insert_resource(hm);
}

pub fn store_audio(
    mut commands: Commands,
    collection: Res<GameAsset>,
    assets: Res<Assets<AudioData>>,
) {
    let mut hm = Store::<AudioData>::new();
    for handle in collection.audio.iter() {
        let Some(asset) = assets.get(handle) else {
            return;
        };
        hm.add(asset.id, asset.clone());
    }
    commands.insert_resource(hm);
}
