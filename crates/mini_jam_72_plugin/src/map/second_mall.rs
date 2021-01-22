use crate::map::{Coordinate, MapData};
use std::collections::HashMap;

pub fn get_second_mall_map() -> MapData {
    let mut path_map = HashMap::default();
    path_map.insert('#', "structure/woodenFloor.png".to_owned());

    return MapData {
        floors: vec!["\
            #############\n\
            ########t####\n\
            ###.#.#######\n\
            #a++++0++++q#\n\
            #####+#+#.###\n\
            #t#####++.#t#\n\
            ###+.###+++##\n\
            ###+++#.#.+##\n\
            #####++++++##\n\
            ##t#.#.####t#\n\
            #############"
            .to_owned()],
        path_map,
    };
}
