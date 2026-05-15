//! 相机模块
//!
//! 负责生成主相机，并根据玩家位置进行跟随。

use bevy::prelude::*;

use crate::app::sets::RenderSet;
use crate::app::states::AppState;

use systems::*;

/// 插件注册：添加相机生成和跟随系统。
///
/// - 在进入 `AppState::Running` 状态时生成相机。
/// - 在 `PostUpdate` 的 `RenderSet::Sync` 阶段，且应用处于 `Running` 状态时，执行相机跟随。
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Building), setup_camera);
    app.add_systems(
        PostUpdate,
        camera_follow_player
            .in_set(RenderSet::Sync)
            .run_if(in_state(AppState::Running)),
    );
}

/// 相机相关组件定义
pub mod components {
    use bevy::prelude::*;

    /// 主相机标记组件，用于标识跟随玩家的相机。
    #[derive(Component, Default, Debug, Clone)]
    pub struct MainCamera;
}

pub mod systems {
    use bevy::prelude::*;

    use crate::logic::player::components::Player;

    use super::components::*;

    /// 设置主相机：生成一个正交 2D 相机，缩放比例设为 0.5，
    /// 并添加 `MainCamera` 标记和 `IsDefaultUiCamera` 组件。
    pub fn setup_camera(mut commands: Commands) {
        let mut projection = OrthographicProjection::default_2d();
        projection.scale = 0.5;
        commands.spawn((
            Camera2d,
            MainCamera,
            IsDefaultUiCamera,
            Projection::Orthographic(projection),
        ));
    }

    /// 相机跟随系统：将主相机的位置同步到玩家实体的全局位置。
    ///
    /// 通过查询 `MainCamera` 的 `Transform` 组件（可变）和玩家实体的 `Transform`，
    /// 将相机的平移设置为玩家当前位置。
    pub fn camera_follow_player(
        camera_quuery: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
        player_query: Single<&Transform, (Changed<Transform>, With<Player>)>,
    ) {
        let mut transform = camera_quuery.into_inner();
        transform.translation = player_query.into_inner().translation;
    }
}
