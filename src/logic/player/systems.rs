use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::input::actions::PlayerAction;
use crate::input::bindings;
use crate::logic::character::components::CharacterBundle;
use crate::logic::player::state_machine;

use super::actions::*;
use super::components::*;

/// 生成玩家实体（应在世界构建阶段调用）。
pub fn spawn_player(mut commands: Commands) {
    let collider = Collider::circle(16.0);
    let character_bundle = CharacterBundle::new(collider);
    let state_machine = state_machine::default_state_machine();
    let input_map = bindings::default_input_map();
    commands.spawn((character_bundle, Player, Idle, input_map, state_machine));
    info!("[Logic/Player] 玩家实体已生成");
}

/// 移动系统：根据输入设置玩家水平速度（仅在 `Moving` 状态时生效）。
pub fn move_system(
    mut q_player: Query<
        (&ActionState<PlayerAction>, &mut LinearVelocity),
        (With<Player>, With<Moving>),
    >,
) {
    let Ok((action_state, mut velocity)) = q_player.single_mut() else {
        return;
    };

    let speed = 300.0;
    let axis = action_state.clamped_axis_pair(&PlayerAction::Move);
    velocity.0 = axis * speed;
    info!("[Logic/Player] Axis: {:?},  Velocity: {:?}", axis, velocity);
}
