use crate::player::{DeathEvent, Player};
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
                transform: Transform {
                    translation: Vec3::new(player.translation.x, player.translation.y + 50., 1.),
                    scale: Vec3::new(2., 2., 2.),
                    ..Default::default()
                },
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
        for player_position in player_query.iter() {
            let direction = player_position.translation - piano.translation;
            if direction == Vec3::zero() {
                continue;
            }
            let movement = direction.normalize() * 50. * time.delta_seconds();
            let scale = 2. + direction.length() / 50.;
            piano.scale = Vec3::new(scale, scale, scale);
            if movement.length() < direction.length() {
                piano.translation += movement;
            } else {
                piano.translation = player_position.translation;
                death_event.send(DeathEvent);
            }
        }
    }
}
