mod actions;
mod audio;
mod map;
mod menu;
mod player;
mod ui;
mod assets;

use crate::audio::InternalAudioPlugin;
use crate::map::{Coordinate, MapPlugin, Maps, PlayerCamera};
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UiPlugin;

use crate::actions::ActionsPlugin;
use bevy::prelude::*;

pub struct GamePlugin;

const STAGE: &str = "game";

#[derive(Clone)]
pub enum AppState {
    Menu,
    InGame,
    RetryGame,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(ClearColor(Color::BLACK))
            .add_resource(State::new(AppState::Menu))
            .add_resource(GameState::default())
            .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
            .add_plugin(MapPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(InternalAudioPlugin)
            .on_state_enter(STAGE, AppState::RetryGame, switch_to_game.system());
    }
}

pub struct GameState {
    pub health: usize,
    pub score: usize,
    pub enemy_health: i32,
    pub current_map: Maps,
    pub player_spawn: Coordinate,
    pub talking_to: Option<Coordinate>,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            health: 20,
            score: 0,
            enemy_health: 1,
            current_map: Maps::Mall,
            player_spawn: Coordinate { x: 200., y: 200. },
            talking_to: None,
        }
    }
}

fn switch_to_game(mut state: ResMut<State<AppState>>) {
    state.set_next(AppState::InGame).unwrap();
}
