mod mall;
mod second_mall;

use crate::map::mall::get_mall_map;
use crate::map::second_mall::get_second_mall_map;
use crate::{AppState, GameState, STAGE};
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(STAGE, AppState::InGame, initialize_map.system())
            .on_state_exit(STAGE, AppState::InGame, break_down_map.system());
    }
}

pub enum Maps {
    Mall,
    SecondMall,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Path,
    Spawn,
    TowerPlot,
    Tower,
    Castle,
    Tree,
    Empty,
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

#[derive(Debug, Default)]
pub struct Map {
    pub height: usize,
    pub width: usize,
    pub tiles: Vec<Vec<Tile>>,
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
}

impl Map {
    pub fn load_map(map_data: MapData) -> Self {
        let mut map = Map {
            height: 0,
            width: 0,
            tiles: vec![],
            tile_size: 64.,
            player_spawn: map_data.player_spawn,
        };

        for map_str in map_data.floors.iter() {
            map.height = map_str.lines().count();
            for (row_index, line) in map_str.lines().enumerate() {
                let _row_index = map.height - row_index - 1;
                let mut row = vec![];
                for (_column_index, char) in line.chars().enumerate() {
                    match char {
                        '0' => row.push(Tile::Tower),
                        '.' => row.push(Tile::TowerPlot),
                        '#' => row.push(Tile::Empty),
                        't' => row.push(Tile::Tree),
                        '+' => row.push(Tile::Path),

                        'a' => row.push(Tile::Spawn),

                        'q' => row.push(Tile::Castle),

                        _ => panic!("unknown map char {}", char),
                    }
                }
                map.tiles.push(row);
            }
        }

        // otherwise my map is head down O.o
        map.tiles.reverse();
        map.width = map.tiles.first().unwrap().len();

        map
    }
}

fn initialize_map(
    commands: &mut Commands,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
    let blank_handle: Handle<Texture> = asset_server.load("blank64x64.png");
    let tower_plot_handle: Handle<Texture> = asset_server.load("towerplot64x64.png");
    let tower_handle: Handle<Texture> = asset_server.load("tower64x64.png");
    let path_handle: Handle<Texture> = asset_server.load("path64x64.png");
    let castle_handle: Handle<Texture> = asset_server.load("castle64x64.png");
    let cloud_handle: Handle<Texture> = asset_server.load("cloud64x64.png");
    let spawn_handle: Handle<Texture> = asset_server.load("spawn.png");

    for row in 0..map.height {
        for column in 0..map.width {
            let tile = &map.tiles[row][column];
            match tile {
                &Tile::Empty => {
                    commands
                        .spawn(SpriteBundle {
                            material: materials.add(blank_handle.clone().into()),
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
                &Tile::TowerPlot => {
                    commands
                        .spawn(SpriteBundle {
                            material: materials.add(tower_plot_handle.clone().into()),
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
                &Tile::Tower => {
                    commands
                        .spawn(SpriteBundle {
                            material: materials.add(tower_handle.clone().into()),
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
                &Tile::Path => {
                    commands
                        .spawn(SpriteBundle {
                            material: materials.add(path_handle.clone().into()),
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
                &Tile::Castle => {
                    commands
                        .spawn(SpriteBundle {
                            material: materials.add(castle_handle.clone().into()),
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
                &Tile::Tree => {
                    commands
                        .spawn(SpriteBundle {
                            material: materials.add(cloud_handle.clone().into()),
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
                &Tile::Spawn => {
                    commands
                        .spawn(SpriteBundle {
                            material: materials.add(spawn_handle.clone().into()),
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
