use bevy::prelude::*;

use crate::logic::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(player::plugin);
    app.add_plugins(world::plugin);
}
