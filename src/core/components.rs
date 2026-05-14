use bevy::prelude::*;

/// 稳定标识符组件，用于存档/网络同步或外部引用。
///
/// 该标识符独立于 Bevy 的 `Entity`，可在程序运行之间持久化。
/// 在需要跨会话识别实体的场景使用（如玩家、NPC、独特物品）。
#[derive(Component, Reflect, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub u64);
