use avian2d::prelude::*;
use bevy::prelude::*;

use my_game::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default().with_length_unit(32.0));

    app.add_plugins(app::plugin);

    app.run();
}
