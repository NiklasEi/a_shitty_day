use crate::player::{DeathEvent, Player};
use crate::AppState;
use bevy::prelude::*;

pub struct FallingPiano;

pub fn throw_piano(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player_query: &Query<&Transform, With<Player>>,
) {
    if let Some(player) = player_query.iter().last() {
        commands
            .spawn(SpriteBundle {
                material: materials.add(
                    asset_server
                        .get_handle(&("textures/".to_owned() + "objects/piano.png")[..])
                        .into(),
                ),
                transform: Transform::from_translation(Vec3::new(
                    player.translation.x + 100.,
                    player.translation.y,
                    3.,
                )),
                ..Default::default()
            })
            .with(FallingPiano);
    }
}

pub fn move_piano(
    time: Res<Time>,
    mut piano_query: Query<&mut Transform, With<FallingPiano>>,
    player_query: Query<&Transform, With<Player>>,
    mut death_event: ResMut<Events<DeathEvent>>,
) {
    for mut piano in piano_query.iter_mut() {
        for mut player_position in player_query.iter() {
            let mut direction = player_position.translation - piano.translation;
            let movement = direction.normalize() * 100. * time.delta_seconds();
            if movement.length() < direction.length() {
                piano.translation += movement;
                // piano.apply_non_uniform_scale(Vec3::new(piano.translation.z.clone(), piano.translation.z.clone(), piano.translation.z.clone()));
            } else {
                death_event.send(DeathEvent);
            }
        }
    }
}
