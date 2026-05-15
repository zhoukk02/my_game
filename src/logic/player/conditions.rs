use bevy::prelude::*;

use crate::input::actions::PlayerAction;
use leafwing_input_manager::prelude::ActionState;

/// 移动轴阈值，小于此值视为无输入
const MOVE_DEADZONE: f32 = 0.01;

/// 检测移动输入是否停止（轴值长度平方小于阈值）
///
/// 用于状态机：`Moving` → `Idle`
pub fn is_move_stopped(
    In(entity): In<Entity>,
    q_action: Query<&ActionState<PlayerAction>>,
) -> bool {
    q_action
        .get(entity)
        .map(|state| state.axis_pair(&PlayerAction::Move).length_squared() < MOVE_DEADZONE)
        .unwrap_or(true)
}
