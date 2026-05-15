//! 玩家逻辑模块
//!
//! 负责玩家的生成、移动控制以及与状态机的交互。
//! 玩家实体使用动态刚体模拟物理移动，但禁用重力，由移动系统直接控制速度。

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::input::actions::PlayerAction;
use crate::input::bindings;
use crate::logic::character::components::CharacterBundle;
use crate::logic::character::components::Direction;
use crate::logic::player::state_machine;

use super::actions::*;
use super::components::*;

/// 生成玩家实体（应在世界构建阶段调用）。
///
/// 创建的玩家实体包含以下组件：
/// - `CharacterBundle`：物理属性（动态刚体、圆形碰撞体、无重力、方向朝下）
/// - `Player`：玩家标记组件
/// - `Idle`：初始状态（空闲）
/// - `InputMap<PlayerAction>`：输入映射
/// - `StateMachine`：状态机（控制动作流转）
///
/// # 注意
/// 玩家生成后即处于 `Idle` 状态，需要输入才能移动。
pub fn spawn_player(mut commands: Commands) {
    let collider = Collider::circle(16.0);
    let character_bundle = CharacterBundle::new(collider);
    let state_machine = state_machine::default_state_machine();
    let input_map = bindings::default_input_map();
    commands.spawn((character_bundle, Player, Idle, input_map, state_machine));
    info!("[Logic/Player] 玩家实体已生成");
}

/// 移动系统：根据输入设置玩家速度（仅在 `Moving` 状态时生效）。
///
/// 该系统读取玩家的移动轴输入（`PlayerAction::Move`），
/// 计算速度向量并更新 `LinearVelocity` 组件，同时根据输入方向更新 `Direction` 组件。
///
/// # 行为
/// - 速度大小固定为 `100.0` 像素/秒。
/// - 方向根据输入轴的主方向确定（优先绝对值大的轴）。
/// - 仅在玩家处于 `Moving` 状态时运行。
///
/// # 参数
/// - `q_player`：查询玩家实体的动作状态、线速度、方向组件，且要求玩家具有 `Moving` 状态标记。
///
/// # 注意
/// 重力已被关闭（`GravityScale(0.0)`），因此速度不会自然衰减，
/// 需要其他系统（例如停止移动时清零速度）或通过阻尼进行处理。
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
    let Ok((action_state, mut velocity, mut direction)) = q_player.single_mut() else {
        return;
    };
    let speed = 100.0;
    let axis = action_state.clamped_axis_pair(&PlayerAction::Move);
    velocity.0 = axis * speed;
    *direction = Direction::from(axis);
    debug!("[Logic/Player] Axis: {:?},  Velocity: {:?}", axis, velocity);
}
