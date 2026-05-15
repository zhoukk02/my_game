use avian2d::prelude::*;
use bevy::prelude::*;

/// 标记组件：表示一个实体是可移动的角色（玩家、NPC、动物等）。
///
/// 该组件不携带数据，仅用于标识，便于其他系统通过 `With<Character>` 进行查询。
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Character;

/// 角色物理组件集合，用于快速生成具有标准物理属性的角色实体。
///
/// 默认包含：
/// - `Character` 标记组件
/// - `RigidBody::Dynamic`：动态刚体，受力和碰撞影响
/// - `LockedAxes::ROTATION_LOCKED`：锁定旋转轴，防止角色因碰撞而倾倒
/// - `GravityScale(0.0)`：禁用重力，由上层逻辑（如移动系统）直接控制速度
///
/// # 示例
/// ```
/// use avian2d::prelude::Collider;
/// use bevy::prelude::Commands;
///
/// fn spawn_player(mut commands: Commands) {
///     commands.spawn(CharacterBundle::new(Collider::circle(16.0)));
/// }
/// ```
#[derive(Bundle)]
pub struct CharacterBundle {
    /// 角色标记组件
    pub character: Character,
    /// 动态刚体
    pub rigid_body: RigidBody,
    /// 碰撞体形状（例如圆形、矩形）
    pub collider: Collider,
    /// 锁定旋转轴（默认禁用旋转）
    pub locked_axes: LockedAxes,
    /// 当前朝向（初始朝下）
    pub direction: Direction,
    /// 重力缩放（默认 0.0，关闭重力）
    pub gravity_scale: GravityScale,
}

impl CharacterBundle {
    /// 创建一个新的角色物理组件组合，使用指定的碰撞体形状。
    ///
    /// 其他字段均为默认值（动态刚体、锁定旋转、无重力、初始方向朝下）。
    pub fn new(collider: Collider) -> Self {
        Self {
            collider,
            character: Character,
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            direction: Direction::Down,
            gravity_scale: GravityScale(0.0),
        }
    }
}

/// 角色面向的方向（主要基于输入轴的主方向）。
///
/// 注意：当输入轴为零向量时，`from` 方法会返回 `Direction::Down`（未定义行为）。
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    /// 从二维向量（如摇杆或键盘轴）计算主方向。
    ///
    /// 规则：
    /// - 优先比较绝对值，取绝对值较大的轴作为主方向。
    /// - 如果 `|x| > |y|`，则左右优先；否则上下优先。
    /// - 零向量会返回 `Direction::Down`（此行为未严格定义，调用者应确保输入非零）。
    pub fn from(axis: Vec2) -> Self {
        if axis.x.abs() > axis.y.abs() {
            if axis.x > 0.0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else {
            if axis.y > 0.0 {
                Direction::Up
            } else {
                Direction::Down
            }
        }
    }
    /// 返回方向的 snake_case 字符串表示（小写）。
    pub fn as_snake_case(&self) -> &'static str {
        match self {
            Direction::Up => "up",
            Direction::Right => "right",
            Direction::Down => "down",
            Direction::Left => "left",
        }
    }
}
