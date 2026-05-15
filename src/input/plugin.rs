use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::actions::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
    info!("[Input] 模块加载完成");
}
