use bevy::prelude::*;

use crate::app::sets::LogicSet;
use crate::app::states::AppState;

use super::messages::*;
use super::systems::*;

pub fn plugin(app: &mut App) {
    app.add_message::<ActionCompleted>()
        .add_message::<MapClosed>()
        .add_message::<PauseResumed>()
        .add_message::<MenuClosed>();

    app.add_systems(OnEnter(AppState::Running), spawn_player)
        .add_systems(
            Update,
            ((move_system).in_set(LogicSet::Simulation),).run_if(in_state(AppState::Running)),
        );
    info!("[Logic/Player] 模块加载完成");
}
