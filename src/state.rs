use crate::prelude::*;

pub struct State {
    pub map: Map,
    pub selected_tile: Coordinate,
    pub active_command: Command,
    pub count: i32,
}

impl State {
    pub fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng, false);
        Self { map: map_builder.map, active_command: Command::Flood, selected_tile: Coordinate::new(0, 0), count: 0 }
    }
}
