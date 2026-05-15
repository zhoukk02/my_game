use bevy::prelude::*;

use super::sets::*;
use super::states::*;

/// 配置全局状态机与系统集顺序的插件函数。
///
/// 该函数负责：
/// - 初始化 `AppState`（应用程序全局状态）
/// - 规定 `Update` 阶段逻辑层系统的执行顺序（`Input` → `Action` → `Simulation`）
/// - 规定 `PostUpdate` 阶段渲染层系统的执行顺序（`Sync` → `Animation` → `Visual` → `Ui` → `Audio`）
pub fn plugin(app: &mut App) {
    app.insert_state(AppState::Loading);

    app.configure_sets(
        Update,
        (LogicSet::Input, LogicSet::Action, LogicSet::Simulation).chain(),
    );

    app.configure_sets(
        PostUpdate,
        (
            RenderSet::Sync,
            RenderSet::Animation,
            RenderSet::Visual,
            RenderSet::Ui,
            RenderSet::Audio,
        )
            .chain(),
    );
    info!("[App] 模块加载完成");
    info!(
        "[App] 系统集顺序已配置：
        Update 逻辑层 Input → Action → Simulation，
        PostUpdate 渲染层 Sync → Animation → Visual → Ui → Audio"
    );
}
