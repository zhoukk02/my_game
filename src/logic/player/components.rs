use bevy::prelude::*;

/// 标记组件：表示一个实体是玩家角色。
///
/// 该组件不携带数据，仅用于标识玩家实体，便于其他系统通过 `With<Player>` 进行查询和操作。
#[derive(Component)]
pub struct Player;
