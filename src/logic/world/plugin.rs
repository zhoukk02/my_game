use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use std::env;

use crate::app::states::AppState;

use super::systems::*;

pub fn plugin(app: &mut App) {
    let mut path = env::current_dir().unwrap();
    path.push("assets/tiled/custom_types.json");
    let fitler = regex::RegexSet::new([format!(r"^my_game::.*")]).unwrap();
    app.add_plugins((
        TiledPlugin(TiledPluginConfig {
            tiled_types_export_file: Some(path),
            tiled_types_filter: TiledFilter::from(fitler),
        }),
        TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default(),
    ));

    app.add_systems(OnEnter(AppState::Building), setup_world);

    app.add_observer(extend_collider_entity);
    app.add_observer(extend_camera_entity);
    app.add_observer(spawn_player_at_spawn_point);
}
