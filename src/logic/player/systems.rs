//! 玩家逻辑模块
//!
//! 负责玩家的移动控制以及与状态机的交互。
//! 玩家实体使用动态刚体模拟物理移动，但禁用重力，由移动系统直接控制速度。

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::input::actions::PlayerAction;
use crate::logic::character::components::Direction;

use super::actions::*;
use super::components::*;
use super::constants::*;

/// 移动系统：根据输入设置玩家速度（仅在 `Moving` 状态时生效）。
///
/// 该系统读取玩家的移动轴输入（`PlayerAction::Move`），
/// 计算速度向量并更新 `LinearVelocity` 组件，同时根据输入方向更新 `Direction` 组件。
///
/// # 行为
/// - 方向根据输入轴的主方向确定（优先绝对值大的轴）。
/// - 仅在玩家处于 `Moving` 状态时运行。
///
/// # 参数
/// - `q_player`：查询玩家实体的动作状态、线速度、方向组件，且要求玩家具有 `Moving` 状态标记。
pub fn move_system(
    mut q_player: Query<
        (
            &ActionState<PlayerAction>,
            &mut LinearVelocity,
            &mut Direction,
        ),
        (With<Player>, With<Moving>),
    >,
) {
    for (action_state, mut velocity, mut direction) in q_player.iter_mut() {
        let axis = action_state.clamped_axis_pair(&PlayerAction::Move);
        velocity.0 = axis * PLAYER_BASE_SPEED;
        *direction = Direction::from(axis);
        debug!(
            "[Logic/Player] 玩家移动中: (方向: {:?}, 速度: {:?})",
            direction, velocity
        );
    }
}
