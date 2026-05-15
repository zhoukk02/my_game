use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use bevy_common_assets::ron::RonAssetPlugin;

use crate::app::states::AppState;

use super::assets::*;
use super::systems::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(RonAssetPlugin::<ItemDefinitionFile>::new(&["items.ron"]));

    app.add_loading_state(
        LoadingState::new(AppState::Loading)
            .continue_to_state(AppState::Building)
            .load_collection::<AsepriteAssets>()
            .load_collection::<SoundAssets>()
            .load_collection::<DefinitionAssets>(),
    );
    let setups = (setup_defs, setup_sounds, setup_aseprites);
    app.add_systems(
        OnEnter(AppState::Building),
        (setups.before(game_ready), game_ready).chain(),
    );
}
