use bevy::prelude::*;

use super::{
    sets::{LogicSet, RenderSet},
    states::AppState,
};

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
    info!("[App] 加载完成");
    info!(
        "[App] 系统集顺序已配置：
        Update 逻辑层 Input → Action → Simulation，
        PostUpdate 渲染层 Sync → Animation → Visual → Ui → Audio"
    );
}
