use crate::actions::Actions;
use crate::map::PlayerCamera;
use crate::{AppState, GameState, STAGE};
use bevy::prelude::*;

pub struct PlayerPlugin;

pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(STAGE, AppState::InGame, spawn_player.system())
            .on_state_update(STAGE, AppState::InGame, move_player.system());
    }
}

fn spawn_player(
    game_state: Res<GameState>,
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(SpriteBundle {
            material: materials.add(asset_server.load("character/mainCharacter.png").into()),
            transform: Transform::from_translation(Vec3::new(
                game_state.player_spawn.x,
                game_state.player_spawn.y,
                1.,
            )),
            ..Default::default()
        })
        .with(Player);
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut transform in camera_query.iter_mut() {
        transform.translation += movement;
    }
    for mut transform in player_query.iter_mut() {
        transform.translation += movement;
    }
}
