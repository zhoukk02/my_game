use bevy::prelude::*;

/// 应用程序全局状态，控制游戏的主要生命周期阶段。
///
/// 状态转换顺序为：`Loading` → `Building` → `Running`。
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AppState {
    /// 资源加载阶段。
    ///
    /// 使用 `bevy_asset_loader` 加载纹理、音频、配置文件等。
    /// 此阶段不应执行任何游戏逻辑。
    #[default]
    Loading,

    /// 世界构建阶段。
    ///
    /// 将已加载的资产转换为运行时数据结构（如 `HashMap`），
    /// 生成地图、玩家、NPC、建筑等实体。
    Building,

    /// 正常运行阶段。
    ///
    /// 游戏主循环，逻辑层和渲染层系统在此阶段运行。
    /// 只有在此阶段才处理玩家输入和游戏模拟。
    Running,
}
