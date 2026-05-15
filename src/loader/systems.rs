use bevy::prelude::*;

use crate::app::states::AppState;

use super::assets::*;
use super::manager::*;

/// 从已加载的物品定义资产构建 `ItemData` 资源。
pub fn setup_defs(
    mut commands: Commands,
    handle: Res<DefinitionAssets>,
    assets: Res<Assets<ItemDefinitionFile>>,
) {
    let Some(file) = assets.get(&handle.item_handle) else {
        return;
    };
    let mut count = 0;
    let mut manager = ItemData::new();
    for kv in &file.data {
        manager.add(kv.id, kv.clone());
        count += 1;
    }
    commands.insert_resource(manager);
    commands.remove_resource::<DefinitionAssets>();
    info!("[Loader] 物品定义加载完成，共 {} 项", count);
}

/// 从预加载的音频句柄列表构建 `SoundData` 资源。
pub fn setup_sounds(
    mut commands: Commands,
    handle: Res<SoundAssets>,
    asset_server: Res<AssetServer>,
) {
    let mut count = 0;
    let mut manager = SoundData::new();
    for handle in handle.sound_handles.iter() {
        let Some(path) = asset_server.get_path(handle) else {
            warn!("[Loader] 无法获取音频句柄的路径: {:?}", handle);
            continue;
        };
        let file_name = path
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        manager.add(file_name, handle.clone());
        count += 1;
    }
    commands.insert_resource(manager);
    commands.remove_resource::<SoundAssets>();
    info!("[Loader] 音频数据加载完成，共 {} 项", count);
}

/// 切换应用状态到 `Running`。
///
/// 该函数应在所有资产数据（物品、音频、地图等）完成资源构建后调用。
pub fn game_ready(mut next_state: ResMut<NextState<AppState>>) {
    info!("[Loader] 资产加载与初始化完成，游戏已就绪");
    next_state.set(AppState::Running);
}
