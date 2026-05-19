use bevy::prelude::*;
use bevy_kira_audio::*;

use crate::app::{AppState, RenderSet};

use super::{
    events::{AudioBgmRequest, AudioSfxRequest},
    resources::{AudioBgmChannel, AudioSfxChannel},
    systems,
};

pub fn plugin(app: &mut App) {
    app.add_plugins(AudioPlugin);

    app.add_message::<AudioBgmRequest>();
    app.add_message::<AudioSfxRequest>();

    app.add_audio_channel::<AudioBgmChannel>();
    app.add_audio_channel::<AudioSfxChannel>();

    app.add_systems(
        PostUpdate,
        (systems::handle_bgm_requests, systems::handle_sfx_requests)
            .in_set(RenderSet::Audio)
            .run_if(in_state(AppState::Running)),
    );
    info!("[Audio] 加载完成");
}
