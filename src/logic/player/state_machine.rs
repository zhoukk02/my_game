use seldom_state::prelude::*;

use crate::input::actions::PlayerAction::*;

use super::actions::*;
use super::conditions::*;
use super::messages::*;

pub fn default_state_machine() -> StateMachine {
    StateMachine::default()
        // Idle ↔ Moving
        .trans::<Idle, _>(is_move_stopped.not(), Moving)
        .trans::<Moving, _>(is_move_stopped, Idle)
        // 从 Idle 进入其他状态
        .trans::<Idle, _>(just_pressed(OpenMap), MapView)
        .trans::<Idle, _>(just_pressed(Interact), Interacting)
        .trans::<Idle, _>(just_pressed(Jump), Jumping)
        .trans::<Idle, _>(just_pressed(UseTool), UsingTool)
        .trans::<Idle, _>(just_pressed(Pause), Paused)
        .trans::<Idle, _>(just_pressed(OpenMenu), MenuView)
        // 从 Moving 进入其他状态
        .trans::<Moving, _>(just_pressed(OpenMap), MapView)
        .trans::<Moving, _>(just_pressed(Interact), Interacting)
        .trans::<Moving, _>(just_pressed(Jump), Jumping)
        .trans::<Moving, _>(just_pressed(UseTool), UsingTool)
        .trans::<Moving, _>(just_pressed(Pause), Paused)
        .trans::<Moving, _>(just_pressed(OpenMenu), MenuView)
        // 动作完成返回 Idle
        .trans::<MapView, _>(on_message::<MapClosed>, Idle)
        .trans::<Interacting, _>(on_message::<ActionCompleted>, Idle)
        .trans::<Jumping, _>(on_message::<ActionCompleted>, Idle)
        .trans::<UsingTool, _>(on_message::<ActionCompleted>, Idle)
        .trans::<Paused, _>(on_message::<PauseResumed>, Idle)
        .trans::<MenuView, _>(on_message::<MenuClosed>, Idle)
}
