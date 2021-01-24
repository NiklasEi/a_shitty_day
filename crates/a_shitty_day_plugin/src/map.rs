mod mall;
mod second_mall;

use crate::map::mall::get_mall_map;
use crate::map::second_mall::get_second_mall_map;
use crate::ui::{CanTalk, ConversationId};
use crate::{AppState, GameState, STAGE};
use bevy::prelude::*;
use std::collections::HashMap;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(TileSpriteHandles::default())
            .on_state_enter(STAGE, AppState::InGame, initialize_map.system())
            .on_state_exit(STAGE, AppState::InGame, break_down_map.system());
    }
}

pub enum Maps {
    Mall,
    SecondMall,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tile {
    pub asset_path: Option<String>,
}

#[derive(Default)]
struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

impl Coordinate {
    pub fn to_vec(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

#[derive(Default)]
struct TileSpriteHandles {
    handles: Vec<HandleUntyped>,
}

pub struct PlayerCamera;

#[derive(Debug, Default)]
pub struct Map {
    pub height: usize,
    pub width: usize,
    pub npcs: Vec<Npc>,
    pub layers: Vec<Vec<Vec<Tile>>>,
    pub colliding_layers: Vec<bool>,
    pub tile_size: f32,
}

pub struct MapTile {
    pub column: usize,
    pub row: usize,
    pub tile: Tile,
}

pub struct Collide {
    pub x: usize,
    pub y: usize,
}

pub struct MapData {
    layers: Vec<String>,
    npcs: Vec<Npc>,
    path_map: HashMap<char, String>,
    colliding_layers: Vec<usize>,
}

#[derive(Debug)]
pub struct Npc {
    conversation_id: Option<ConversationId>,
    position: Coordinate,
    asset: Option<String>,
}

pub type NpcPosition = Coordinate;

impl Map {
    pub fn load_map(map_data: MapData) -> Self {
        let mut map = Map {
            height: 0,
            width: 0,
            npcs: map_data.npcs,
            layers: vec![],
            colliding_layers: vec![],
            tile_size: 32.,
        };

        for (floor_index, map_str) in map_data.layers.iter().enumerate() {
            let mut floor = vec![];
            map.height = map_str.lines().count();
            for (row_index, line) in map_str.lines().enumerate() {
                let _row_index = map.height - row_index - 1;
                let mut row = vec![];
                for (_column_index, char) in line.chars().enumerate() {
                    if let Some(path) = map_data.path_map.get(&char) {
                        row.push(Tile {
                            asset_path: Some(path.clone()),
                        })
                    } else {
                        row.push(Tile { asset_path: None })
                    }
                }
                floor.push(row);
            }
            // otherwise my map is head down O.o
            floor.reverse();
            map.colliding_layers
                .push(map_data.colliding_layers.contains(&floor_index));
            map.layers.push(floor);
        }
        map.width = map.layers.first().unwrap().first().unwrap().len();

        map
    }
}

fn initialize_map(
    commands: &mut Commands,
    mut tile_sprite_handles: ResMut<TileSpriteHandles>,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<&mut Transform, With<PlayerCamera>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    tile_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
    let map = match game_state.current_map {
        Maps::Mall => Map::load_map(get_mall_map()),
        Maps::SecondMall => Map::load_map(get_second_mall_map()),
    };
    if let Some(mut camera) = camera_query.iter_mut().last() {
        camera.translation = Vec3::new(game_state.player_spawn.x, game_state.player_spawn.y, 10.);
    } else {
        commands
            .spawn(Camera2dBundle {
                transform: Transform::from_translation(Vec3::new(
                    game_state.player_spawn.x,
                    game_state.player_spawn.y,
                    10.,
                )),
                ..Camera2dBundle::default()
            })
            .with(PlayerCamera);
    }
    render_map(commands, &map, &asset_server, &mut materials);

    commands.insert_resource(map);
}

fn render_map(
    commands: &mut Commands,
    map: &Map,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    for (layer_index, layer) in map.layers.iter().enumerate() {
        let collide = map
            .colliding_layers
            .get(layer_index)
            .unwrap_or(&false)
            .clone();
        for row in 0..map.height {
            for column in 0..map.width {
                let tile = &layer[row][column];
                if let Some(path) = &tile.asset_path {
                    let sprite = SpriteBundle {
                        material: materials.add(
                            asset_server
                                .get_handle(&("textures/".to_owned() + path)[..])
                                .into(),
                        ),
                        transform: Transform::from_translation(Vec3::new(
                            column as f32 * map.tile_size,
                            row as f32 * map.tile_size,
                            0.,
                        )),
                        ..Default::default()
                    };
                    let tile = MapTile {
                        column,
                        row,
                        tile: tile.clone(),
                    };
                    if collide {
                        commands
                            .spawn(sprite)
                            .with(tile)
                            .with(Collide { x: column, y: row });
                    } else {
                        commands.spawn(sprite).with(tile);
                    }
                }
            }
        }
    }

    for npc in map.npcs.iter() {
        if let Some(asset) = &npc.asset {
            let sprite = SpriteBundle {
                material: materials.add(asset_server.load(&asset[..]).into()),
                transform: Transform::from_translation(Vec3::new(
                    npc.position.x,
                    npc.position.y,
                    1.,
                )),
                ..Default::default()
            };
            if let Some(id) = npc.conversation_id {
                commands
                    .spawn(sprite)
                    .with(CanTalk { id })
                    .with(NpcPosition {
                        x: npc.position.x,
                        y: npc.position.y,
                    });
            } else {
                commands.spawn(sprite).with(NpcPosition {
                    x: npc.position.x,
                    y: npc.position.y,
                });
            }
        } else {
            if let Some(id) = npc.conversation_id {
                println!("spawning {:?}", id);
                commands.spawn(CanTalk { id }).with(NpcPosition {
                    x: npc.position.x,
                    y: npc.position.y,
                });
            }
        }
    }
}

fn break_down_map(commands: &mut Commands, tile_query: Query<Entity, With<Tile>>) {
    for entity in tile_query.iter() {
        commands.despawn(entity);
    }
}
