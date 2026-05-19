use bevy::prelude::*;

use my_game::{app::AppState, audio::AudioBgmRequest, prelude::*};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(app::plugin);

    app.add_plugins(loader::plugin);
    // app.add_plugins(input::plugin);
    app.add_plugins(audio::plugin);

    app.add_systems(OnEnter(AppState::Running), play_bgm);

    app.run();
}

pub fn play_bgm(mut writer: MessageWriter<AudioBgmRequest>) {
    writer.write(AudioBgmRequest::Play { id: 1 });
}
