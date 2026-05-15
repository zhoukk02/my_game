use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_asset_loader::prelude::*;

use serde::Deserialize;

use super::definitions::*;

/// 物品定义文件资产，包含所有物品的配置数据。
///
/// 对应 RON 文件格式：`{ data: [ ... ] }`
#[derive(Asset, TypePath, Deserialize)]
pub struct ItemDefinitionFile {
    /// 物品定义列表
    pub data: Vec<ItemDefinition>,
}

/// 静态配置资产集合（物品、NPC、作物等定义文件）。
#[derive(Resource, AssetCollection)]
pub struct DefinitionAssets {
    /// 物品定义文件句柄
    #[asset(path = "defs/defs.items.ron")]
    pub item_handle: Handle<ItemDefinitionFile>,
}

/// 音频资产集合，预加载所有 `.ogg` 文件。
#[derive(Resource, AssetCollection)]
pub struct SoundAssets {
    /// 音频源句柄列表（从 `assets/sounds/` 目录递归收集）
    #[asset(path = "sounds", collection(typed))]
    pub sound_handles: Vec<Handle<AudioSource>>,
}

/// Aseprite 资源集合，用于预加载 `aseprites/` 目录下的所有动画文件。
#[derive(Resource, AssetCollection)]
pub struct AsepriteAssets {
    /// 所有已加载的 Aseprite 句柄列表
    #[asset(path = "aseprites", collection(typed))]
    pub aseprite_handles: Vec<Handle<Aseprite>>,
}
