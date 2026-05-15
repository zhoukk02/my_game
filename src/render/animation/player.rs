use bevy::prelude::*;

use crate::app::sets::RenderSet;
use crate::app::states::AppState;

use systems::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        (player_animation_idle, player_animation_move)
            .in_set(RenderSet::Animation)
            .run_if(in_state(AppState::Running)),
    );
    app.add_observer(extend_player_entity);
}

pub mod systems {
    use bevy::prelude::*;
    use bevy_aseprite_ultra::prelude::*;

    use crate::loader::manager::AsepriteData;
    use crate::logic::character::components::Direction;
    use crate::logic::player::actions::*;
    use crate::logic::player::components::Player;

    pub fn extend_player_entity(
        player_created: On<Insert, Player>,
        mut commands: Commands,
        resource: Res<AsepriteData>,
    ) {
        let Some(handle) = resource.get("player.aseprite") else {
            warn!("[Render/Animation] 未找到动画资源 'player.aseprite'，玩家动画无法加载");
            return;
        };
        commands.entity(player_created.entity).insert((
            Sprite::default(),
            AseAnimation {
                animation: Animation::tag("walk-up")
                    .with_repeat(AnimationRepeat::Loop)
                    .with_direction(AnimationDirection::Forward)
                    .with_speed(2.0),
                aseprite: handle.clone(),
            },
        ));
    }

    pub fn player_animation_idle(
        mut query: Query<(&mut AseAnimation, &Direction), (Added<Idle>, With<Player>)>,
    ) {
        for (mut ase, _direction) in query.iter_mut() {
            ase.animation.play_loop("idle");
        }
    }

    pub fn player_animation_move(
        mut query: Query<(&mut AseAnimation, &Direction), (Added<Moving>, With<Player>)>,
    ) {
        for (mut ase, direction) in query.iter_mut() {
            let tag = format!("walk-{}", direction.as_snake_case());
            ase.animation.play_loop(tag);
        }
    }
}
