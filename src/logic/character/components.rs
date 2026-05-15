use avian2d::prelude::*;
use bevy::prelude::*;

/// 标记组件：表示一个实体是可移动的角色（玩家、NPC、动物等）。
///
/// 该组件不携带数据，仅用于标识，便于其他系统通过 `With<Character>` 进行查询。
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Character;

/// 角色物理组件集合，用于快速生成具有标准物理属性的角色实体。
///
/// 包含：
/// - `Character` 标记组件
/// - `RigidBody::Dynamic`：动态刚体，受力和碰撞影响
/// - 指定的 `Collider` 碰撞体（例如圆形、矩形）
/// - `LockedAxes::ROTATION_LOCKED`：锁定旋转轴，防止角色因碰撞而倾倒
#[derive(Bundle)]
pub struct CharacterBundle {
    /// 角色标记组件
    pub character: Character,
    /// 动态刚体
    pub rigid_body: RigidBody,
    /// 碰撞体形状（由调用者提供）
    pub collider: Collider,
    /// 锁定旋转轴（默认禁用旋转，保持角色直立）
    pub locked_axes: LockedAxes,
}

impl CharacterBundle {
    /// 创建一个新的角色物理组件组合。
    pub fn new(collider: Collider) -> Self {
        Self {
            collider,
            character: Character,
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
        }
    }
}
