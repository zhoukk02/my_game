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
pub fn setup_world(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // 地图锚点：左上角对齐世界坐标原点 (0,0,0)，便于按像素精确放置
        TilemapAnchor::TopLeft,
        // 各图层在 Z 轴上的间隔量，避免渲染时出现深度冲突（闪烁），此处设 100 确保分层清晰
        TiledMapLayerZOffset(100.0),
        // 渲染管线的自定义参数。
        TilemapRenderSettings {
            // 启用 Y 轴排序，让 Y 值较大的物体（靠下）显示在 Y 值较小的物体之上。
            // 对于等距或横版游戏，可以正确实现前后遮挡效果。
            y_sort: true,
            // 每个渲染区块的尺寸（瓦片数）。此处为 32×32 瓦片。
            // 由于瓦片尺寸为 16×16 像素，单个区块像素大小为 512×512。
            render_chunk_size: UVec2::splat(32),
        },
        TiledWorld(asset_server.load("tiled/my_game.world")),
        // 配置 Tiled 地图的物理后端（Avian2D）
        TiledPhysicsSettings::<TiledPhysicsAvianBackend> {
            // 只从 Tiled 中名称为 "collision" 的对象层（Object Layer）生成碰撞体
            // FIXME: 当前 Tiled 导出的图层名大小写敏感，必须手动确保名称全小写，否则无法识别
            objects_layer_filter: TiledFilter::Names(vec![String::from("collision")]),
            // 忽略瓦片本身（tile 对象）的碰撞，避免为每个瓦片自动生成碰撞体
            tiles_objects_filter: TiledFilter::None,
            // 其他物理参数使用默认值（例如碰撞体的密度、摩擦力等）
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
/// 1. 获取出生点实体的 `GlobalTransform` 坐标。
/// 2. 创建玩家实体（包含 `PlayerBundle`、`Idle` 初始状态及出生点变换）。
pub fn spawn_player_at_spawn_point(
    player_created: On<Add, PlayerSpawnPoint>,
    spawn_point_query: Query<&Transform, With<PlayerSpawnPoint>>,
    mut commands: Commands,
) {
    let Ok(transform) = spawn_point_query.get(player_created.entity) else {
        error!("[World] 未能获取玩家出生点信息, 无法生成玩家");
        return;
    };

    let collider = Collider::circle(16.0);
    let player_bundle = PlayerBundle::new(collider);
    commands.spawn((transform.clone(), player_bundle, Idle));
    info!(
        "[World] 玩家已在出生点生成, 坐标 {:?}",
        transform.translation
    );
}
