use crate::assets::background_music;
use crate::{AppState, STAGE};
use bevy::prelude::{AppBuilder, AssetServer, Handle, IntoSystem, Plugin, Res};
use bevy_kira_audio::{Audio, AudioPlugin, AudioSource};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin)
            .on_state_enter(STAGE, AppState::InGame, start_background.system())
            .on_state_exit(STAGE, AppState::InGame, stop_sound.system());
    }
}

fn start_background(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music: Handle<AudioSource> = asset_server.load(background_music());
    audio.play_looped(music);
}

fn stop_sound(audio: Res<Audio>) {
    audio.stop();
}
