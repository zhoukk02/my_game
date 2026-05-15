//! 世界设置与碰撞体扩展模块
//!
//! 负责加载 Tiled 世界（.world 文件），设置物理碰撞层以及自动扩展相机和碰撞体实体的功能。

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::logic::player::actions::Idle;
use crate::logic::player::components::PlayerBundle;
use crate::render::camera::components::MainCamera;

use super::components::*;

/// 设置游戏世界：加载 Tiled 地图世界（.world 文件）并配置物理后端。
///
/// 生成一个实体，包含：
/// - `TilemapAnchor::BottomCenter`：图块锚点位于底部中心（影响坐标系对齐）。
/// - `TiledWorld`：加载 `tiled/my_game.world` 资源。
/// - `TiledPhysicsSettings`：为物理后端 `TiledPhysicsAvianBackend` 配置碰撞检测设置。
///
/// # 注意
/// `objects_layer_filter` 当前通过名字 `"collision"` 来识别碰撞图层。
/// 需要注意 Tiled 中图层名的大小写（目前 FIXME 提示需要将名称转换为小写才能正确匹配）。
pub fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TilemapAnchor::BottomCenter,
        TiledWorld(asset_server.load("tiled/my_game.world")),
        TiledPhysicsSettings::<TiledPhysicsAvianBackend> {
            // FIXME: 需要将名称转换为小写，否则无法正常识别
            objects_layer_filter: TiledFilter::Names(vec![String::from("collision")]),
            tiles_objects_filter: TiledFilter::None,
            ..default()
        },
    ));
}

/// 扩展碰撞体实体：为 Tiled 创建的碰撞体添加静态刚体组件。
///
/// 当 Tiled 世界生成碰撞体（`ColliderCreated` 事件）时，
/// 为该碰撞体的所属实体插入 `RigidBody::Static`，使其成为物理世界中不可移动的静态障碍物。
pub fn extend_collider_entity(
    collider_created: On<TiledEvent<ColliderCreated>>,
    mut commands: Commands,
) {
    commands
        .entity(collider_created.event().origin)
        .insert((RigidBody::Static,));
}

/// 扩展相机实体：为主相机添加视差滚动支持。
///
/// 当带有 `MainCamera` 组件的实体被插入时，自动为其添加 `TiledParallaxCamera` 组件，
/// 使其能够与 Tiled 地图的视差层配合，产生景深感。
pub fn extend_camera_entity(camera_created: On<Insert, MainCamera>, mut commands: Commands) {
    commands
        .entity(camera_created.entity)
        .insert(TiledParallaxCamera);
}
/// 当 `PlayerSpawnPoint` 组件被添加到实体时，在该出生点位置生成玩家实体。
///
/// # 触发时机
/// 在地图加载或世界构建阶段，出生点实体被创建并添加 `PlayerSpawnPoint` 组件后触发。
///
/// # 行为说明
/// 1. 获取出生点实体的 `Transform` 坐标。
/// 2. 创建玩家实体（包含 `PlayerBundle`、`Idle` 初始状态及出生点变换）。
pub fn spawn_player_at_spawn_point(
    player_created: On<Add, PlayerSpawnPoint>,
    spawn_point_query: Query<&Transform, With<PlayerSpawnPoint>>,
    mut commands: Commands,
) {
    let Ok(transform) = spawn_point_query.get(player_created.entity) else {
        return;
    };
    let collider = Collider::circle(16.0);
    let player_bundle = PlayerBundle::new(collider);
    commands.spawn((player_bundle, Idle, transform.clone()));
    info!("[World] 玩家已在出生点生成");
}
