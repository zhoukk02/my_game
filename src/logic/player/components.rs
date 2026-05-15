use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;

use crate::input::actions::PlayerAction;
use crate::input::bindings;
use crate::logic::character::components::CharacterBundle;

use super::state_machine;

/// 标记组件：表示一个实体是玩家角色。
///
/// 该组件不携带数据，仅用于标识玩家实体，便于其他系统通过 `With<Player>` 进行查询和操作。
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    character_bundle: CharacterBundle,
    input_map: InputMap<PlayerAction>,
    state_machine: StateMachine,
}

impl PlayerBundle {
    pub fn new(collider: Collider) -> Self {
        Self {
            player: Player,
            character_bundle: CharacterBundle::new(collider),
            input_map: bindings::default_input_map(),
            state_machine: state_machine::default_state_machine(),
        }
    }
}
