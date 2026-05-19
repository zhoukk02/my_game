use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use bevy_common_assets::ron::RonAssetPlugin;

use crate::{
    app::AppState,
    data::{AudioData, ItemData},
};

use super::{assets::GameAsset, systems};

pub fn plugin(app: &mut App) {
    app.add_plugins(RonAssetPlugin::<ItemData>::new(&["items.ron"]));
    app.add_plugins(RonAssetPlugin::<AudioData>::new(&["audio.ron"]));

    app.add_loading_state(
        LoadingState::new(AppState::Loading)
            .continue_to_state(AppState::Building)
            .load_collection::<GameAsset>(),
    );

    let setups = (systems::store_item, systems::store_audio);
    app.add_systems(
        OnEnter(AppState::Building),
        (systems::game_ready, setups.before(systems::game_ready)),
    );
    info!("[Loader] 加载完成");
}
