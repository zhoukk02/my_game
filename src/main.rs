use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use my_game::prelude::*;
use seldom_state::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_plugins(StateMachinePlugin::default());
    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(AsepriteUltraPlugin);

    app.add_plugins(app::plugin);
    app.add_plugins(core::plugin);
    app.add_plugins(loader::plugin);
    app.add_plugins(input::plugin);

    app.add_plugins(logic::plugin);
    app.add_plugins(render::plugin);

    app.run();
}
