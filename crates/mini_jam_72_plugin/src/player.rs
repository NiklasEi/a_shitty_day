use crate::actions::Actions;
use crate::map::{Collide, Map, PlayerCamera};
use crate::{AppState, GameState, STAGE};
use bevy::prelude::*;
use std::f32::consts::PI;
use crate::ui::HideConversation;

pub struct PlayerPlugin;

pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(STAGE, AppState::InGame, spawn_player.system())
            .on_state_update(STAGE, AppState::InGame, move_player.system())
            .on_state_exit(STAGE, AppState::InGame, remove_player.system());
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
    map: Res<Map>,
    mut hide_conversation: ResMut<Events<HideConversation>>,
    mut game_state: ResMut<GameState>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    collider_query: Query<&Collide>,
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
    for mut player_transform in player_query.iter_mut() {
        player_transform.rotation = Quat::from_rotation_z(
            -1. * actions
                .player_movement
                .unwrap()
                .angle_between(Vec2::new(0., 1.)),
        );
        let x =
            ((player_transform.translation.x + movement.x + map.tile_size / 2.) / map.tile_size) as usize;
        let y =
            ((player_transform.translation.y + movement.y + map.tile_size / 2.) / map.tile_size) as usize;
        for collide in collider_query.iter() {
            if collide.x == x && collide.y == y {
                return;
            }
        }
        player_transform.translation += movement;
        for mut transform in camera_query.iter_mut() {
            transform.translation = player_transform.translation;
        }
        if let Some(pos) = &game_state.talking_to {
            if pos.to_vec().distance(Vec2::new(player_transform.translation.x, player_transform.translation.y)) > 32. {
                hide_conversation.send(HideConversation);
                game_state.talking_to = None;
            }
        }
    }
}

fn remove_player(
    commands: &mut Commands,
    player_query: Query<Entity, With<Player>>,) {
    for player in player_query.iter() {
        commands.despawn(player);
    }
}
