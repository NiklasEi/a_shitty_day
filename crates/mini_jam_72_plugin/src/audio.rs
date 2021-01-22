use crate::{AppState, STAGE};
use bevy::prelude::{
    AppBuilder, AssetServer, Handle, IntoSystem, Plugin, Res, ResMut,
    Time, Timer,
};
use bevy_improved_audio::{Audio, AudioPlugin, AudioSource};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin)
            .add_resource(BackgroundTimer::from_seconds(3. * 60., true))
            .on_state_enter(STAGE, AppState::InGame, start_background.system())
            .on_state_update(STAGE, AppState::InGame, background.system())
            .on_state_exit(STAGE, AppState::InGame, break_down_audio.system());
    }
}

type BackgroundTimer = Timer;

fn start_background(asset_server: Res<AssetServer>, audio: Res<Audio<AudioSource>>) {
    let music: Handle<AudioSource> = asset_server.load("sounds/background.mp3");
    audio.play_in_channel(music, "background".to_owned());
}

fn background(
    mut timer: ResMut<BackgroundTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    timer.tick(time.delta_seconds());
    if timer.just_finished() {
        let music = asset_server.load("sounds/background.mp3");
        audio.play_in_channel(music, "background".to_owned());
    }
}

fn break_down_audio(audio: Res<Audio>) {
    audio.drop_channel("background".to_owned());
}
