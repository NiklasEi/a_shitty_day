use crate::map::{Coordinate, MapData};

pub fn get_mall_map() -> MapData {
    return MapData {
        floors: vec!["\
            #############\n\
            ########t####\n\
            ###.#.#######\n\
            #a++++0++++q#\n\
            #####+#+#.###\n\
            #t#+++#++.#t#\n\
            ###+.###+++##\n\
            ###+++#.#.+##\n\
            #####++++++##\n\
            ##t#.#.####t#\n\
            #############"
            .to_owned()],
        player_spawn: Coordinate { x: 100., y: 100. },
    };
}
