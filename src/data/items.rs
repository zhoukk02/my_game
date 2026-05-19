use bevy::prelude::*;

use serde::Deserialize;

use crate::app::Id;

#[derive(Asset, TypePath, Deserialize, Debug, Clone)]
pub struct ItemData {
    pub id: Id,
    pub name: String,
    pub category: ItemCategory,
    pub description: String,
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
