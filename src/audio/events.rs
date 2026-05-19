use bevy::prelude::*;

use crate::app::Id;

#[derive(Message)]
pub enum AudioBgmRequest {
    Play { id: Id },
    Stop,
    Pause,
    Resume,
    SetVolume(f32),
}

#[derive(Message)]
pub enum AudioSfxRequest {
    Play { id: Id },
}
