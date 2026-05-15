use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::logic::character::components::Direction;
use crate::logic::player::actions::*;
use crate::logic::player::components::Player;

pub fn extend_player_entity(
    player_created: On<Insert, Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let path = "textures/characters/player/player.aseprite";
    commands.entity(player_created.entity).insert((
        Sprite::default(),
        AseAnimation {
            animation: Animation::tag("walk-up")
                .with_repeat(AnimationRepeat::Loop)
                .with_direction(AnimationDirection::Forward)
                .with_speed(2.0),
            aseprite: asset_server.load(path),
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
        match direction {
            Direction::Up => {
                ase.animation.play_loop("walk-up");
            }
            Direction::Right => {
                ase.animation.play_loop("walk-right");
            }
            Direction::Down => {
                ase.animation.play_loop("walk-down");
            }
            Direction::Left => {
                ase.animation.play_loop("walk-left");
            }
        }
    }
}
