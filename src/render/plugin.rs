use bevy::prelude::*;

use crate::render::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(camera::plugin);
    app.add_plugins(animation::plugin);
}
