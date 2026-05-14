use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::actions::*;

/// 创建玩家输入的默认映射配置。
///
/// 支持键盘、鼠标和手柄，绑定规则如下：
/// - `Move`：WASD / 左摇杆（带死区灵敏度）
/// - `Dash`：左 Shift / 手柄北键 (Y/△)
/// - `Jump`：空格 / 手柄南键 (A/×)
/// - `UseTool`：鼠标左键 / 手柄西键 (X/□)
/// - `Interact`：鼠标右键 / 手柄东键 (B/○)
/// - `OpenMenu`：I 键 / 手柄 Start 键
/// - `OpenMap`：M 键 / 手柄 Select 键
/// - `Pause`：P 键 / 手柄 Select + Start 组合键
pub fn default_input_map() -> InputMap<PlayerAction> {
    let sensitivity = 2.0;
    let dead_zone = CircleDeadZone::new(0.15);
    let left_stick = GamepadStick::LEFT
        .sensitivity(sensitivity)
        .with_processor(DualAxisProcessor::CircleDeadZone(dead_zone));

    let pause_chord = ButtonlikeChord::new([GamepadButton::Select, GamepadButton::Start]);

    InputMap::default()
        .with_dual_axis(PlayerAction::Move, VirtualDPad::wasd())
        .with_dual_axis(PlayerAction::Move, left_stick)
        .with(PlayerAction::Dash, KeyCode::ShiftLeft)
        .with(PlayerAction::Dash, GamepadButton::North)
        .with(PlayerAction::Jump, KeyCode::Space)
        .with(PlayerAction::Jump, GamepadButton::South)
        .with(PlayerAction::UseTool, MouseButton::Left)
        .with(PlayerAction::UseTool, GamepadButton::West)
        .with(PlayerAction::Interact, MouseButton::Right)
        .with(PlayerAction::Interact, GamepadButton::East)
        .with(PlayerAction::OpenMenu, KeyCode::KeyI)
        .with(PlayerAction::OpenMenu, GamepadButton::Start)
        .with(PlayerAction::OpenMap, KeyCode::KeyM)
        .with(PlayerAction::OpenMap, GamepadButton::Select)
        .with(PlayerAction::Pause, KeyCode::KeyP)
        .with(PlayerAction::Pause, pause_chord)
}
