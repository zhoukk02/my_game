use bevy::prelude::*;

use super::components::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Id>();

    info!("[Core] 模块加载完成");
}
