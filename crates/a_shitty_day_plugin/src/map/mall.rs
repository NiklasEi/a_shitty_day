use crate::map::{Coordinate, MapData, Npc};
use std::collections::HashMap;

pub fn get_mall_map() -> MapData {
    let mut path_map = HashMap::default();
    path_map.insert('#', "structure/woodenFloor.png".to_owned());
    path_map.insert('w', "structure/wallblue.png".to_owned());
    path_map.insert('e', "structure/escalator3.png".to_owned());
    path_map.insert('p', "structure/stoneFloor.png".to_owned());
    path_map.insert('1', "objects/bed_1.png".to_owned());
    path_map.insert('2', "objects/bed_2.png".to_owned());
    path_map.insert('3', "objects/bed_3.png".to_owned());
    path_map.insert('4', "objects/bed_4.png".to_owned());

    return MapData {
        layers: vec![
            "\
            #########p\n\
            ####12###p\n\
            ####43###p\n\
            #########p\n\
            ########ep\n\
            #########p\n\
            #########p\n\
            #########p\n\
            #########p\n\
            #########p\n\
            #########p"
                .to_owned(),
            "\
            wwwwwwwww.\n\
            w.......w.\n\
            w.......w.\n\
            w.......w.\n\
            w.........\n\
            w.......w.\n\
            w.......w.\n\
            w.......w.\n\
            w.......w.\n\
            w.......w.\n\
            wwwwwwwww."
                .to_owned(),
        ],
        path_map,
        colliding_layers: vec![1],
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
