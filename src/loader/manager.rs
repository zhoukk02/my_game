use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use std::collections::HashMap;

use super::definitions::*;

/// 物品数据资源，用于运行时高效查询物品定义。
///
/// 内部以 `HashMap` 存储物品 ID 到定义的映射。
#[derive(Resource)]
pub struct ItemData {
    m: HashMap<u64, ItemDefinition>,
}

impl ItemData {
    /// 创建一个空的物品数据容器。
    pub fn new() -> Self {
        Self { m: HashMap::new() }
    }

    /// 添加或更新一个物品定义。
    ///
    /// # 参数
    /// - `id`: 物品唯一标识符
    /// - `def`: 物品定义
    pub fn add(&mut self, id: u64, def: ItemDefinition) {
        self.m.insert(id, def);
    }

    /// 根据 ID 获取物品定义的引用。
    pub fn get(&self, id: &u64) -> Option<&ItemDefinition> {
        self.m.get(id)
    }
}

/// 音频数据资源，用于根据名称获取预加载的音频源句柄。
///
/// 内部以 `HashMap` 存储音频名称到音频源句柄的映射。
#[derive(Resource)]
pub struct SoundData {
    m: HashMap<String, Handle<AudioSource>>,
}

impl SoundData {
    /// 创建一个空的音频数据容器。
    pub fn new() -> Self {
        Self { m: HashMap::new() }
    }

    /// 添加音频源句柄，关联到指定的名称。
    ///
    /// # 参数
    /// - `name`: 音频名称
    /// - `handle`: 已加载的音频源句柄
    pub fn add(&mut self, name: String, handle: Handle<AudioSource>) {
        self.m.insert(name, handle);
    }

    /// 根据名称获取音频源句柄的引用。
    pub fn get(&self, name: &str) -> Option<&Handle<AudioSource>> {
        self.m.get(name)
    }
}

/// Aseprite 动画数据资源，用于运行时根据名称快速获取动画句柄。
///
/// 内部以 `HashMap` 存储动画名称到 Aseprite 句柄的映射。
#[derive(Resource)]
pub struct AsepriteData {
    m: HashMap<String, Handle<Aseprite>>,
}

impl AsepriteData {
    /// 创建一个空的动画数据容器。
    pub fn new() -> Self {
        Self { m: HashMap::new() }
    }

    /// 添加动画句柄，关联到指定名称。
    ///
    /// # 参数
    /// - `name`：动画标识符（通常为不含扩展名的文件名）
    /// - `handle`：已加载的 Aseprite 句柄
    pub fn add(&mut self, name: String, handle: Handle<Aseprite>) {
        self.m.insert(name, handle);
    }

    /// 根据名称获取动画句柄的引用。
    pub fn get(&self, name: &str) -> Option<&Handle<Aseprite>> {
        self.m.get(name)
    }
}
