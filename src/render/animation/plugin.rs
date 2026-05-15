use bevy::prelude::*;

use crate::app::sets::RenderSet;
use crate::app::states::AppState;

use super::systems::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        (player_animation_idle, player_animation_move)
            .in_set(RenderSet::Animation)
            .run_if(in_state(AppState::Running)),
    );
    app.add_observer(extend_player_entity);
}
