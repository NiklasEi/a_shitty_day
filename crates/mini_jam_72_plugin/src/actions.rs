use crate::{AppState, STAGE};
use bevy::prelude::*;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Actions>().on_state_update(
            STAGE,
            AppState::InGame,
            set_movement_actions.system(),
        );
    }
}

#[derive(Default)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

fn set_movement_actions(mut actions: ResMut<Actions>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_released(KeyCode::W)
        || keyboard_input.just_released(KeyCode::S)
        || keyboard_input.pressed(KeyCode::W)
        || keyboard_input.pressed(KeyCode::S)
        || keyboard_input.just_released(KeyCode::A)
        || keyboard_input.just_released(KeyCode::D)
        || keyboard_input.pressed(KeyCode::A)
        || keyboard_input.pressed(KeyCode::D)
    {
        let mut player_movement = Vec2::zero();

        if keyboard_input.just_released(KeyCode::W) || keyboard_input.just_released(KeyCode::S) {
            if keyboard_input.pressed(KeyCode::W) {
                player_movement.y = 1.;
            } else if keyboard_input.pressed(KeyCode::S) {
                player_movement.y = -1.;
            } else {
                player_movement.y = 0.;
            }
        } else if keyboard_input.just_pressed(KeyCode::W) {
            player_movement.y = 1.;
        } else if keyboard_input.just_pressed(KeyCode::S) {
            player_movement.y = -1.;
        } else {
            player_movement.y = actions.player_movement.unwrap_or(Vec2::zero()).y;
        }

        if keyboard_input.just_released(KeyCode::D) || keyboard_input.just_released(KeyCode::A) {
            if keyboard_input.pressed(KeyCode::D) {
                player_movement.x = 1.;
            } else if keyboard_input.pressed(KeyCode::A) {
                player_movement.x = -1.;
            } else {
                player_movement.x = 0.;
            }
        } else if keyboard_input.just_pressed(KeyCode::D) {
            player_movement.x = 1.;
        } else if keyboard_input.just_pressed(KeyCode::A) {
            player_movement.x = -1.;
        } else {
            player_movement.x = actions.player_movement.unwrap_or(Vec2::zero()).x;
        }

        if player_movement != Vec2::zero() {
            player_movement = player_movement.normalize();
            actions.player_movement = Some(player_movement);
        }
    } else {
        actions.player_movement = None;
    }
}
