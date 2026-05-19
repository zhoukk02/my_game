use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::data::{AudioData, Store};

use super::{
    events::{AudioBgmRequest, AudioSfxRequest},
    resources::{AudioBgmChannel, AudioSfxChannel},
};

pub fn handle_bgm_requests(
    mut requests: MessageReader<AudioBgmRequest>,
    channel: Res<AudioChannel<AudioBgmChannel>>,
    resource: Res<Store<AudioData>>,
    asset_server: Res<AssetServer>,
) {
    for request in requests.read() {
        match request {
            AudioBgmRequest::Play { id } => {
                let Some(s) = resource.get(id) else {
                    continue;
                };

                channel
                    .play(asset_server.load(&s.path))
                    .with_volume(s.volume)
                    .with_playback_rate(s.playback_rate)
                    .with_panning(s.panning)
                    .looped();
            }
            AudioBgmRequest::Stop => {
                channel.stop();
            }
            AudioBgmRequest::Pause => {
                channel.pause();
            }
            AudioBgmRequest::Resume => {
                channel.resume();
            }
            AudioBgmRequest::SetVolume(volmue) => {
                channel.set_volume(*volmue);
            }
        }
    }
}

pub fn handle_sfx_requests(
    mut requests: MessageReader<AudioSfxRequest>,
    channel: Res<AudioChannel<AudioSfxChannel>>,
    resource: Res<Store<AudioData>>,
    asset_server: Res<AssetServer>,
) {
    for request in requests.read() {
        match request {
            AudioSfxRequest::Play { id } => {
                let Some(s) = resource.get(id) else {
                    continue;
                };

                channel
                    .play(asset_server.load(&s.path))
                    .with_volume(s.volume)
                    .with_playback_rate(s.playback_rate)
                    .with_panning(s.panning);
            }
        }
    }
}
