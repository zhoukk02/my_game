use bevy::prelude::*;

/// 逻辑层系统集，定义游戏逻辑在 `Update` 阶段的执行顺序。
///
/// 各变体应按以下顺序执行：
/// `Input` → `Action` → `Simulation`
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogicSet {
    /// 原始输入处理阶段。
    ///
    /// 读取 `leafwing` 的 `ActionState`，不修改游戏状态。
    Input,
    /// 输入解析与状态转换阶段。
    ///
    /// 将原始输入转换为 `seldom_state` 触发器，处理交互请求。
    /// 此阶段结束后应完成所有状态机转换。
    Action,
    /// 核心模拟阶段。
    ///
    /// 执行移动、物理、背包、时间、天气等业务逻辑。
    Simulation,
}

/// 渲染层系统集，定义表现层在 `PostUpdate` 阶段的执行顺序。
///
/// 各变体应按以下顺序执行：
/// `Sync` → `Animation` → `Visual` → `Ui` → `Audio`
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RenderSet {
    /// 同步逻辑数据到渲染组件。
    ///
    /// 例如将 `Transform` 位置同步到 `Sprite` 或 `Mesh`。
    /// 此阶段必须最先执行，确保后续渲染使用最新数据。
    Sync,
    /// 动画驱动阶段。
    ///
    /// 根据逻辑状态（如 `CharActionState`）更新 `AnimationPlayer`。
    Animation,
    /// 视觉效果更新阶段。
    ///
    /// 更新粒子系统、天气特效（雨、雪）、后处理等。
    Visual,
    /// UI 刷新阶段。
    ///
    /// 读取背包、时间、生命值等逻辑数据，更新 UI 节点。
    /// 通常需要在前序渲染完成后进行，避免覆盖。
    Ui,
    /// 音频播放阶段。
    ///
    /// 处理 `PlaySoundEvent`，实际播放音效与背景音乐。
    /// 不依赖其它渲染结果，置于最后执行。
    Audio,
}
