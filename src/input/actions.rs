use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// 玩家可执行的输入动作。
///
/// 与 `leafwing-input-manager` 配合，每个变体代表一种玩家意图，
/// 可通过 `ActionState<PlayerAction>` 查询状态。
#[derive(
    Actionlike, Reflect, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum PlayerAction {
    /// 移动（双轴输入）
    #[default]
    Move,
    /// 冲刺
    Dash,
    /// 跳跃
    Jump,
    /// 交互（对话、拾取、调查）
    Interact,
    /// 使用当前选中的工具
    UseTool,
    /// 打开主菜单（背包、设置等）
    OpenMenu,
    /// 打开地图
    OpenMap,
    /// 暂停游戏
    Pause,
}
