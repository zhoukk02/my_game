use bevy::prelude::*;

use super::player;

pub fn plugin(app: &mut App) {
    app.add_plugins(player::plugin);
}
