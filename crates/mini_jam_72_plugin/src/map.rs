mod mall;
mod second_mall;

use crate::map::mall::get_mall_map;
use crate::map::second_mall::get_second_mall_map;
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
pub struct  Tile {
    pub asset_path: Option<String>
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

#[derive(Default)]
struct TileSpriteHandles {
    handles: Vec<HandleUntyped>
}

#[derive(Debug, Default)]
pub struct Map {
    pub height: usize,
    pub width: usize,
    pub floors: Vec<Vec<Vec<Tile>>>,
    pub tile_size: f32,
    pub player_spawn: Coordinate,
}

pub struct MapTile {
    pub column: usize,
    pub row: usize,
    pub tile: Tile,
}

pub struct MapData {
    floors: Vec<String>,
    player_spawn: Coordinate,
    path_map: HashMap<char, String>
}

impl Map {
    pub fn load_map(map_data: MapData) -> Self {
        let mut map = Map {
            height: 0,
            width: 0,
            floors: vec![],
            tile_size: 32.,
            player_spawn: map_data.player_spawn,
        };

        for map_str in map_data.floors.iter() {
            let mut floor = vec![];
            map.height = map_str.lines().count();
            for (row_index, line) in map_str.lines().enumerate() {
                let _row_index = map.height - row_index - 1;
                let mut row = vec![];
                for (_column_index, char) in line.chars().enumerate() {
                    if let Some(path) = map_data.path_map.get(&char) {
                        row.push(Tile {
                            asset_path: Some(path.clone())
                        })
                    } else {
                        row.push(Tile {
                            asset_path: None
                        })
                    }
                }
                floor.push(row);
            }
            // otherwise my map is head down O.o
            floor.reverse();
            map.floors.push(floor);
        }
        map.width = map.floors.first().unwrap().first().unwrap().len();

        map
    }
}

fn initialize_map(
    commands: &mut Commands,
    mut tile_sprite_handles: ResMut<TileSpriteHandles>,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    tile_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
    let map = match game_state.current_map {
        Maps::Mall => Map::load_map(get_mall_map()),
        Maps::SecondMall => Map::load_map(get_second_mall_map()),
    };
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            map.player_spawn.x,
            map.player_spawn.y,
            10.,
        )),
        ..Camera2dBundle::default()
    });
    render_map(commands, &map, &asset_server, &mut materials);

    commands.insert_resource(map);
}

fn render_map(
    commands: &mut Commands,
    map: &Map,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    for floor in map.floors.iter() {
        for row in 0..map.height {
            for column in 0..map.width {
                let tile = &floor[row][column];
                if let Some(path) = &tile.asset_path {
                    commands
                        .spawn(SpriteBundle {
                            material: materials.add(asset_server.get_handle(&("textures/".to_owned() + path)[..]).into()),
                            transform: Transform::from_translation(Vec3::new(
                                column as f32 * map.tile_size,
                                row as f32 * map.tile_size,
                                0.,
                            )),
                            ..Default::default()
                        })
                        .with(MapTile {
                            column,
                            row,
                            tile: tile.clone(),
                        });
                }
            }
        }
    }
}

fn break_down_map(commands: &mut Commands, tile_query: Query<Entity, With<Tile>>) {
    for entity in tile_query.iter() {
        commands.despawn(entity);
    }
}
