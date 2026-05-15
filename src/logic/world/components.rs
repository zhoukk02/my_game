use bevy::prelude::*;

/// 玩家出生点标记组件。
///
/// 附加于地图中的特定实体（通常从 Tiled 对象层读取），
/// 用于标识玩家初次出现或复活时的位置。
///
/// 该组件自动要求目标实体拥有 `Transform` 组件，
/// 以便直接读取或设置出生坐标。
#[derive(Reflect, Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[require(Transform)]
#[reflect(Component)]
pub struct PlayerSpawnPoint;
