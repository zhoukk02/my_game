use serde::Deserialize;

/// 物品定义（从 RON 文件反序列化）。
#[derive(Deserialize, Debug, Clone)]
pub struct ItemDefinition {
    /// 物品唯一标识符
    pub id: u64,
    /// 物品所属分类
    pub category: ItemCategory,
    /// 物品显示名称
    pub name: String,
    /// 物品描述文本
    pub description: String,
    /// 单个堆叠的最大数量
    pub max_stack: u32,
    /// 购买价格（商店买入）
    pub buy_price: u32,
    /// 出售价格（卖给商店）
    pub sell_price: u32,
}

/// 物品分类枚举。
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ItemCategory {
    // ----- 基础资源 -----
    /// 农作物（番茄、小麦等）
    Crop,
    /// 动物产品（牛奶、鸡蛋、羊毛）
    Product,
    /// 采集品（野果、木材、石头）
    Forage,
    /// 鱼类
    Fish,

    // ----- 加工品 -----
    /// 工匠产品（奶酪、酒、布料）
    ArtisanGood,
    /// 烹饪食物
    CookedFood,

    // ----- 工具与装备 -----
    /// 工具（锄头、水壶、斧头）
    Tool,
    /// 装备（头盔、胸甲、戒指）
    Equipment,

    // ----- 特殊 -----
    /// 种子
    Seed,
    /// 任务物品
    QuestItem,
    /// 配方/蓝图
    Blueprint,
    /// 货币
    Currency,
}
