use crate::map::{Coordinate, MapData, Npc};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tiled::parse;
use tiled::LayerData::Finite;
use tiled::PropertyValue::BoolValue;

pub fn get_mall_map() -> MapData {
    let file = File::open(&Path::new("assets/map/mall.tmx")).unwrap();
    let reader = BufReader::new(file);
    let map = parse(reader).unwrap();

    let mut path_map = HashMap::default();
    for set in map.tilesets.iter() {
        for tile in set.tiles.iter() {
            path_map.insert(
                set.first_gid + tile.id,
                tile.images.first().unwrap().source.clone(),
            );
        }
    }

    let mut layers = vec![];
    for layer in map.layers.iter() {
        let mut current_layer = vec![];
        if let Finite(tiles) = &layer.tiles {
            for row in tiles {
                let mut current_row = vec![];
                for tile in row {
                    current_row.push(tile.gid);
                }
                current_layer.push(current_row);
            }
        }
        layers.push(current_layer);
    }

    return MapData {
        layers,
        path_map,
        height: map.height,
        width: map.width,
        colliding_layers: map
            .layers
            .iter()
            .enumerate()
            .filter(|(_index, layer)| {
                if let Some(BoolValue(collide)) = layer.properties.get("collide") {
                    return collide.clone();
                }
                false
            })
            .map(|(index, _layer)| index)
            .collect(),
        npcs: vec![
            Npc {
                asset: Some("textures/objects/coin.png".to_owned()),
                conversation_id: Some(1),
                position: Coordinate { x: 144., y: 288. },
            },
            Npc {
                asset: Some("character/character2.png".to_owned()),
                conversation_id: Some(2),
                position: Coordinate {
                    x: 7. * 32.,
                    y: 7. * 32.,
                },
            },
            Npc {
                asset: Some("character/character3.png".to_owned()),
                conversation_id: None,
                position: Coordinate {
                    x: 2. * 32.,
                    y: 6. * 32.,
                },
            },
            Npc {
                asset: Some("character/character4.png".to_owned()),
                conversation_id: Some(3),
                position: Coordinate {
                    x: 3. * 32.,
                    y: 2. * 32.,
                },
            },
        ],
    };
}
