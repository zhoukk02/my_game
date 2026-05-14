use avian2d::prelude::*;
use bevy::prelude::*;

use my_game::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(PhysicsPlugins::default());

    app.add_plugins(app::plugin);
    app.add_plugins(core::plugin);

    app.add_plugins(input::plugin);

    app.run();
}
