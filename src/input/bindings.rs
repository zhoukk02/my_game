use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::actions::*;

/// 创建玩家输入的默认映射配置。
///
/// 支持键盘、鼠标和手柄，绑定规则如下：
/// - `Move`：WASD / 左摇杆（带死区灵敏度）
/// - `OpenMap`：M 键 / 手柄北键 (Y/△)
/// - `Interact` F 键 / 手柄东键 (B/○)
/// - `Jump`：空格 / 手柄南键 (A/×)
/// - `UseTool`：鼠标左键 / 手柄西键 (X/□)
/// - `Pause`：P 键 / 手柄 Select 键
/// - `OpenMenu`：I 键 / 手柄 Start 键
pub fn default_input_map() -> InputMap<PlayerAction> {
    let sensitivity = 2.0;
    let dead_zone = CircleDeadZone::new(0.15);
    let left_stick = GamepadStick::LEFT
        .sensitivity(sensitivity)
        .with_processor(DualAxisProcessor::CircleDeadZone(dead_zone));

    InputMap::default()
        // 移动（双轴输入）
        .with_dual_axis(PlayerAction::Move, VirtualDPad::wasd())
        .with_dual_axis(PlayerAction::Move, left_stick)
        // 打开地图
        .with(PlayerAction::OpenMap, KeyCode::KeyM)
        .with(PlayerAction::OpenMap, GamepadButton::North)
        // 交互（对话、拾取、调查）
        .with(PlayerAction::Interact, KeyCode::KeyF)
        .with(PlayerAction::Interact, GamepadButton::East)
        // 跳跃
        .with(PlayerAction::Jump, KeyCode::Space)
        .with(PlayerAction::Jump, GamepadButton::South)
        // 使用当前选中的工具
        .with(PlayerAction::UseTool, MouseButton::Left)
        .with(PlayerAction::UseTool, GamepadButton::West)
        // 暂停游戏
        .with(PlayerAction::Pause, KeyCode::KeyP)
        .with(PlayerAction::Pause, GamepadButton::Select)
        // 打开主菜单（背包、设置等）
        .with(PlayerAction::OpenMenu, KeyCode::KeyI)
        .with(PlayerAction::OpenMenu, GamepadButton::Start)
}
