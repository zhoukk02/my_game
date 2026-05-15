use bevy::prelude::*;

/// 动作完成消息。用于通知状态机：冲刺、跳跃、交互、使用工具等动作已结束，应切换回空闲状态。
#[derive(Message, Debug, Clone)]
pub struct ActionCompleted;

/// 地图关闭完成消息。在地图 UI 关闭动画播放完毕后发送，通知状态机退出地图视图模式。
#[derive(Message, Debug, Clone)]
pub struct MapClosed;

/// 暂停恢复消息。当用户从暂停状态返回游戏时发送，用于将状态机从暂停模式切回正常运行。
#[derive(Message, Debug, Clone)]
pub struct PauseResumed;

/// 菜单关闭完成消息。在菜单 UI 关闭动画播放完毕后发送，通知状态机退出菜单视图模式。
#[derive(Message, Debug, Clone)]
pub struct MenuClosed;
