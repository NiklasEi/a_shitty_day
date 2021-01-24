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
            .add_startup_system(start_background.system())
            .add_system(background.system());
    }
}

type BackgroundTimer = Timer;

fn start_background(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio<AudioSource>>,
) {
    commands.insert_resource(BackgroundTimer::from_seconds(31.008, true));
    let music: Handle<AudioSource> = asset_server.load(background_music());
    audio.play_in_channel(music.clone(), "background".to_owned());
}

fn background(
    mut timer: ResMut<BackgroundTimer>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    audio: Res<Audio>,
) {
    timer.tick(time.delta_seconds());
    if timer.just_finished() {
        let music: Handle<AudioSource> = asset_server.load(background_music());
        audio.play_in_channel(music, "background".to_owned());
    }
}
