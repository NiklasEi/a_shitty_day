mod piano;

use crate::events::piano::{move_piano, throw_piano, FallingPiano};
use crate::player::Player;
use crate::{AppState, GameState, STAGE};
use bevy::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ShittyEvents>()
            .add_resource(ShittyEventsState::default())
            .on_state_update(STAGE, AppState::InGame, trigger_events.system())
            .on_state_update(STAGE, AppState::InGame, handle_events.system())
            .on_state_update(STAGE, AppState::InGame, move_piano.system())
            .on_state_exit(STAGE, AppState::InGame, clean_up_events.system());
    }
}

#[derive(Default)]
pub struct ShittyEventsState {
    triggered: Vec<ShittyEvents>,
    perm_triggered: Vec<ShittyEvents>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ShittyEvents {
    Piano,
}

fn trigger_events(
    mut shitty_events: ResMut<Events<ShittyEvents>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for player_position in player_query.iter() {
        if player_position.translation.x > 272. {
            shitty_events.send(ShittyEvents::Piano);
        }
    }
}

fn handle_events(
    commands: &mut Commands,
    mut game_state: ResMut<GameState>,
    mut event_state: ResMut<ShittyEventsState>,
    mut events: Local<EventReader<ShittyEvents>>,
    event: Res<Events<ShittyEvents>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Some(event) = events.latest(&event) {
        match event {
            ShittyEvents::Piano => {
                if !event_state.triggered.contains(event)
                    && !event_state.perm_triggered.contains(event)
                {
                    game_state.can_walk = false;
                    event_state.triggered.push(event.clone());

                    throw_piano(commands, &asset_server, &mut materials, &player_query)
                }
            }
        }
    }
}

fn clean_up_events(
    commands: &mut Commands,
    mut events_state: ResMut<ShittyEventsState>,
    piano_query: Query<Entity, With<FallingPiano>>,
) {
    events_state.triggered.clear();
    for piano in piano_query.iter() {
        commands.despawn(piano);
    }
}
