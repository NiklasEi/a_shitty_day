use crate::assets::background_music;
use crate::{AppState, STAGE};
use bevy::ecs::Commands;
use bevy::prelude::{
    AppBuilder, AssetServer, Handle, IntoSystem, Plugin, Res, ResMut, Time, Timer,
};
use bevy_improved_audio::{Audio, AudioPlugin, AudioSource};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin)
            .add_resource(BackgroundTimer::from_seconds(31.008, true))
            .on_state_enter(STAGE, AppState::InGame, start_background.system())
            .on_state_update(STAGE, AppState::InGame, background.system())
            .on_state_exit(STAGE, AppState::InGame, break_down_audio.system());
    }
}

type BackgroundTimer = Timer;

struct AudioHandles {
    background: Handle<AudioSource>,
}

fn start_background(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio<AudioSource>>,
) {
    let music: Handle<AudioSource> = asset_server.load(background_music());
    audio.play_in_channel(music.clone(), "background".to_owned());
    commands.insert_resource(AudioHandles { background: music });
}

fn background(
    handles: Res<AudioHandles>,
    mut timer: ResMut<BackgroundTimer>,
    time: Res<Time>,
    audio: Res<Audio>,
) {
    timer.tick(time.delta_seconds());
    if timer.just_finished() {
        audio.play_in_channel(handles.background.clone(), "background".to_owned());
    }
}

fn break_down_audio(audio: Res<Audio>) {
    audio.drop_channel("background".to_owned());
}
