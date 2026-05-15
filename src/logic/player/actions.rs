use bevy::prelude::*;

/// 空闲状态。角色静止，可切换至移动、冲刺、跳跃、交互、使用工具等动作。
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Idle;

/// 移动状态。角色正在行走或奔跑，可切换至空闲、冲刺、跳跃、交互、使用工具等动作。
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Moving;

/// 地图视图模式。此时玩家无法移动或执行其他动作，UI 显示地图。
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MapView;

/// 交互动作状态。用于拾取、打开宝箱、对话等。
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interacting;

/// 跳跃状态。角色处于空中，落地后自动切换回空闲。
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Jumping;

/// 使用工具状态。例如挥锄头、浇水等，动作完成后自动返回空闲。
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UsingTool;

/// 暂停模式。游戏逻辑冻结，UI 显示暂停界面。
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Paused;

/// 菜单视图模式。例如打开背包、设置界面，角色停止移动。
#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MenuView;
