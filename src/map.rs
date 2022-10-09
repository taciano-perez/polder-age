use std::{collections::VecDeque};

use crate::prelude::*;

pub const SEA_BOTTOM: i32 = 1;
pub const SEA_LEVEL: i32 = 4;
pub const MAX_HEIGHT:i32 = 7;
const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Flatland,
    Water,
    Dike,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct TileEntry {
    pub x: i32,
    pub y: i32,
    pub water_level: i32,
}

impl TileEntry {
    pub fn new(x: i32, y: i32, water_level: i32) -> Self {
        Self {
            x,
            y,
            water_level,
        }
    }
}

impl Ord for TileEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on water_level.
        // In case of a tie we compare coordinates - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.water_level.cmp(&self.water_level)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for TileEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MapTile {
    height: i32,
    water: i32,
    improvement: TileImprovement
}

impl MapTile {
    pub fn new(height: i32) -> Self {
        let water = if height <= SEA_LEVEL {SEA_LEVEL - height} else {0};
        Self {
            height,
            water,
            improvement: TileImprovement::None,
        }
    }
    pub fn tile_type(&self)  -> TileType {
        if self.water > 0 { TileType::Water }
        else { 
            match self.improvement {
                TileImprovement::Dike => TileType::Dike,
                _ => TileType::Flatland
            }
        }
    }
    pub fn increase_height(&mut self) {
        self.height = cmp::min(self.height+1, MAX_HEIGHT);
    }
    pub fn lower_height(&mut self) {
        self.height = cmp::max(self.height-1, SEA_BOTTOM);
    }
    pub fn add_water(&mut self, amount: i32) {
            // let start = self.water;
        // SEA_BOTTOM never increases or decreases
        // if self.height > SEA_BOTTOM {
            self.water = cmp::min(self.water+amount, MAX_HEIGHT-self.height);
        // }
        // self.water += amount;
        // println!("Added {} water, from {} to {}", amount, start, self.water);
    }
pub fn remove_water(&mut self, amount: i32) {
        // let start = self.water;
        // if self.height > SEA_BOTTOM {
            self.water = cmp::max(self.water-amount, 0);
        // }
        // println!("Removed {} water, from {} to {}", amount, start, self.water);
    }
    pub fn set_improvement(&mut self, improvement: TileImprovement) {
        self.improvement = improvement;
    }
    fn water_level(&self) -> i32 {
        self.water + self.height
    }
    pub fn screen_glyph(&self) -> char {
        let radix = 10;
        match char::from_digit(self.water_level().wrapping_abs() as u32, radix) {
            Some(glyph) => { glyph },
            None => { 'N' },
        }
    //    match self.tile_type() {
    //         TileType::Flatland => ' ',
    //         TileType::Water => '~',
    //         TileType::Dike => '/',
    //    }
    }
    pub fn screen_color_bg(&self) -> (u8, u8, u8) {
        match self.tile_type() {
            TileType::Water => {
                match self.height {
                    1 => DARK_BLUE,
                    2 => BLUE_VIOLET,
                    3 => BLUE,
                    _ => LIGHT_BLUE,
                }
            },
            TileType::Flatland => {
                match self.height {
                    5 => LIGHT_GREEN,
                    6 => GREEN,
                    7 => DARK_GREEN,
                    _ => GREEN_YELLOW,
                }
            },
            TileType::Dike => {
                match self.height {
                    4 => LIGHT_GREEN,
                    5 => GREEN,
                    6 => DARK_GREEN,
                    _ => GREEN_YELLOW,
                }
            },
        }
    }
    pub fn screen_color_fg(&self) -> (u8, u8, u8) {
        match self.screen_color_bg() {
            LIGHT_GREEN => DARK_GREEN,
            GREEN => DARK_GREEN,
            DARK_GREEN => LIGHT_GREEN,
            GREEN_YELLOW => DARK_GREEN,
            _ => WHITE,
        }
    }
}

// #[derive(Copy, Clone, PartialEq, Debug)]
// pub enum WaterEvent {
//     Flood,
//     Drain,
//     Recalculate,
// }

pub struct Map {
    pub tiles: Vec<MapTile>,
    pub river_source: Coordinate,
    pub connected_to_sea: Vec<usize>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y*SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new(river_source: Coordinate) -> Self {
        Self {
            tiles: vec![MapTile::new(0); NUM_TILES],
            river_source,
            connected_to_sea: Vec::new(),
        }
    }
    pub fn increase_height(&mut self, x: i32, y: i32) {
        let idx = map_idx(x, y);
        self.tiles[idx].increase_height();
        // if self.tiles[idx].water > 0 { self.trigger_event(x, y, WaterEvent::Drain, 1); }
    }
    pub fn lower_height(&mut self, x: i32, y: i32) {
        let idx = map_idx(x, y);
        self.tiles[idx].lower_height();
        self.recalculate_water(Coordinate::new(x, y));
        //self.flood_if_needed(x, y);
        // self.trigger_event(x, y, WaterEvent::Recalculate, 1);
    }
    pub fn build_dike(&mut self, x: i32, y: i32) {
        let idx = map_idx(x, y);
        if !self.valid_coordinates(x, y) || self.tiles[idx].water > 0 { return };
        self.tiles[idx].set_improvement(TileImprovement::Dike);
    }
    pub fn valid_coordinates(&self, x: i32, y: i32) -> bool {
        (0..SCREEN_WIDTH).contains(&x) && (0..SCREEN_HEIGHT).contains(&y)
    }
    pub fn recalculate_water(&mut self, initial_pos: Coordinate) {
        let mut to_be_recalculated = VecDeque::new();
        let mut visited = vec![vec![false; (SCREEN_HEIGHT+1) as usize]; (SCREEN_WIDTH+1) as usize];
        to_be_recalculated.push_back(initial_pos);
        while let Some(pos) = to_be_recalculated.pop_front() {
            // println!("Dequeued {}, {}", pos.x, pos.y);
            self.recalculate_water_recursive(pos, &mut to_be_recalculated, &mut visited);
        }
    }
    
    fn recalculate_water_recursive(&mut self, pos: Coordinate, to_be_recalculated: &mut VecDeque<Coordinate>, visited: &mut [Vec<bool>]) {
        let x = pos.x;
        let y = pos.y;
        // sanity check
        if !self.valid_coordinates(x, y) { return }
        let x_ = x as usize;
        let y_ = y as usize;
        if visited[x_][y_] { return };
        visited[x_][y_] = true;

        let idx = map_idx(x, y);
        if self.tiles[idx].height >= MAX_HEIGHT || self.tiles[idx].tile_type() == TileType::Dike { return; };

        // valid neighbors, ordered by water level
        let mut neighbors: BinaryHeap<TileEntry> = BinaryHeap::new();
        if let Some(pos) = self.valid_neighbor(x+1, y) { neighbors.push(TileEntry::new(pos.x, pos.y, self.tiles[map_idx(pos.x, pos.y)].water_level())) };
        if let Some(pos) = self.valid_neighbor(x-1, y) { neighbors.push(TileEntry::new(pos.x, pos.y, self.tiles[map_idx(pos.x, pos.y)].water_level())) };
        if let Some(pos) = self.valid_neighbor(x, y+1) { neighbors.push(TileEntry::new(pos.x, pos.y, self.tiles[map_idx(pos.x, pos.y)].water_level())) };
        if let Some(pos) = self.valid_neighbor(x, y-1) { neighbors.push(TileEntry::new(pos.x, pos.y, self.tiles[map_idx(pos.x, pos.y)].water_level())) };

        // pick next neighbor
        let mut neighbor_with_lower_water_level = neighbors.pop();
        if neighbor_with_lower_water_level == None { return; }

        while self.tiles[idx].water > 0 && self.tiles[idx].water_level() > SEA_LEVEL && neighbor_with_lower_water_level != None {
            let nentry = neighbor_with_lower_water_level.unwrap();
            if nentry.water_level > self.tiles[idx].water_level() { break; }
            if visited[nentry.x as usize][nentry.y as usize] {
                neighbor_with_lower_water_level = neighbors.pop();
                continue;
            }
            println!("Tile {}, {}, water: {} neighbor: {}, {}, water: {}", x, y, self.tiles[idx].water, nentry.x, nentry.y, nentry.water_level);
            // transfer from origin to neighbor
            println!("Transferring water from {}, {} to {}, {}", x, y, nentry.x, nentry.y);
            self.tiles[idx].remove_water(1);
            self.tiles[map_idx(nentry.x, nentry.y)].add_water(1);
            // println!("Queuing {}, {}", nentry.x, nentry.y);
            to_be_recalculated.push_back(Coordinate::new(nentry.x, nentry.y));
            neighbor_with_lower_water_level = neighbors.pop();
        }
        // if we have to move water from a neighbor to this one?
        // while (there's a higher water_level neighbor with water) {
        //     transfer water from neighbor to origin
        //     recalculate_water(neighbor)
        // }
    }


    /* NEW CODE */
    pub fn recalculate_sea_level(&mut self, new_sea_level: i32) {
        self.connected_to_sea = Vec::new();
        let mut visited = Vec::new();
        let mut to_visit = Vec::new();
        to_visit.push(Coordinate::new(0, 0));
        while let Some(pos) = to_visit.pop() {
            self.recalculate_sea_level_recursive(new_sea_level, pos, &mut visited, &mut to_visit);
        }
    }
    fn recalculate_sea_level_recursive(&mut self, new_sea_level: i32, pos: Coordinate, visited: &mut Vec<usize>, to_visit: &mut Vec<Coordinate>) {
        if self.valid_coordinates(pos.x, pos.y) {
            let pos_idx = map_idx(pos.x, pos.y);
            if !visited.contains(&pos_idx) {
                if self.tiles[pos_idx].water_level() < new_sea_level {
                    // increase water level
                    while self.tiles[pos_idx].water_level() != new_sea_level {
                        self.tiles[pos_idx].add_water(1);
                    }
                    // it's connected
                    self.connected_to_sea.push(pos_idx);
                } else if self.tiles[pos_idx].water_level() > new_sea_level {
                    // decrease water level
                    while self.tiles[pos_idx].water_level() != new_sea_level && self.tiles[pos_idx].water > 0 {
                        self.tiles[pos_idx].remove_water(1);
                        println!("decreasing {},{} by one, water_level: {}, new_sea_level: {}, water: {}", pos.x, pos.y, self.tiles[pos_idx].water_level(), new_sea_level, self.tiles[pos_idx].water);
                    }
                    // is it still connected?
                    if self.tiles[pos_idx].water > 0 {
                        self.connected_to_sea.push(pos_idx);
                    }
                }
            }
            visited.push(pos_idx);
            // visit neighbors
            if self.valid_coordinates(pos.x-1, pos.y) && !visited.contains(&map_idx(pos.x-1, pos.y)) { to_visit.push(Coordinate { x: pos.x-1, y: pos.y }); }
            if self.valid_coordinates(pos.x+1, pos.y) && !visited.contains(&map_idx(pos.x+1, pos.y)) { to_visit.push(Coordinate { x: pos.x+1, y: pos.y }); }
            if self.valid_coordinates(pos.x, pos.y-1) && !visited.contains(&map_idx(pos.x, pos.y-1)) { to_visit.push(Coordinate { x: pos.x, y: pos.y-1 }); }
            if self.valid_coordinates(pos.x, pos.y+1) && !visited.contains(&map_idx(pos.x, pos.y+1)) { to_visit.push(Coordinate { x: pos.x, y: pos.y+1 }); }
        }
    }
    fn valid_neighbor(&self, x: i32, y: i32) -> Option<Coordinate> {
        if self.valid_coordinates(x, y) {
            Some(Coordinate::new(x, y))
        } else {
            None
        }
    }
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                let tile = self.tiles[idx];
                ctx.set(x, y, tile.screen_color_fg(), tile.screen_color_bg(), to_cp437(tile.screen_glyph()));
            }
        }
    }
}

#[cfg(test)]
mod map_tests {
    use crate::prelude::*;

    #[test]
    fn increase_sea_level() {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng, false);
        let mut map = map_builder.map;

        // ENSURE MAP HAS A DRY CANAL AND LAKE
        // canal
        let tile_idx_canal1 = map_idx(23, 19);
        let tile_idx_canal2 = map_idx(24, 19);
        let tile_idx_canal3 = map_idx(25, 19);
        map.tiles[tile_idx_canal1] = MapTile::new(SEA_LEVEL);
        map.tiles[tile_idx_canal2] = MapTile::new(SEA_LEVEL);
        map.tiles[tile_idx_canal3] = MapTile::new(SEA_LEVEL);
        // lake
        let tile_idx_lake_center = map_idx(26, 19);
        let tile_idx_lake_up = map_idx(26, 18);
        let tile_idx_lake_down = map_idx(26, 20);
        let tile_idx_lake_right = map_idx(27, 19);
        map.tiles[tile_idx_lake_center] = MapTile::new(SEA_LEVEL);
        map.tiles[tile_idx_lake_up] = MapTile::new(SEA_LEVEL);
        map.tiles[tile_idx_lake_down] = MapTile::new(SEA_LEVEL);
        map.tiles[tile_idx_lake_right] = MapTile::new(SEA_LEVEL);
        // embankment
        let tile_idx_emb_canal_top1 = map_idx(23, 18);
        let tile_idx_emb_canal_top2 = map_idx(24, 18);
        let tile_idx_emb_canal_top3 = map_idx(25, 18);
        let tile_idx_emb_canal_bottom1 = map_idx(23, 20);
        let tile_idx_emb_canal_bottom2 = map_idx(24, 20);
        let tile_idx_emb_canal_bottom3 = map_idx(25, 20);
        map.tiles[tile_idx_emb_canal_top1] = MapTile::new(MAX_HEIGHT);
        map.tiles[tile_idx_emb_canal_top2] = MapTile::new(MAX_HEIGHT);
        map.tiles[tile_idx_emb_canal_top3] = MapTile::new(MAX_HEIGHT);
        map.tiles[tile_idx_emb_canal_bottom1] = MapTile::new(MAX_HEIGHT);
        map.tiles[tile_idx_emb_canal_bottom2] = MapTile::new(MAX_HEIGHT);
        map.tiles[tile_idx_emb_canal_bottom3] = MapTile::new(MAX_HEIGHT);

        let lake_center_tile = map.tiles[tile_idx_lake_center];
        assert_eq!(lake_center_tile.water, 0);  // no water

        // increase sea level
        map.recalculate_sea_level(SEA_LEVEL+1);
        // check that canal and lake have been filled
        assert_eq!(map.tiles[tile_idx_canal1].water, 1);
        assert_eq!(map.tiles[tile_idx_canal2].water, 1);
        assert_eq!(map.tiles[tile_idx_canal3].water, 1);
        assert_eq!(map.tiles[tile_idx_lake_center].water, 1);
        assert_eq!(map.tiles[tile_idx_lake_up].water, 1);
        assert_eq!(map.tiles[tile_idx_lake_down].water, 1);
        assert_eq!(map.tiles[tile_idx_lake_right].water, 1);
        // check that embankment has NOT been filled
        assert_eq!(map.tiles[tile_idx_emb_canal_top1].water, 0);
        assert_eq!(map.tiles[tile_idx_emb_canal_top2].water, 0);
        assert_eq!(map.tiles[tile_idx_emb_canal_top3].water, 0);
        assert_eq!(map.tiles[tile_idx_emb_canal_bottom1].water, 0);
        assert_eq!(map.tiles[tile_idx_emb_canal_bottom2].water, 0);
        assert_eq!(map.tiles[tile_idx_emb_canal_bottom3].water, 0);

    }
    #[test]
    fn decrease_sea_level() {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng, false);
        let mut map = map_builder.map;

        // ENSURE MAP HAS A DRY CANAL AND LAKE
        // canal
        let tile_idx_canal1 = map_idx(23, 19);
        let tile_idx_canal2 = map_idx(24, 19);
        let tile_idx_canal3 = map_idx(25, 19);
        map.tiles[tile_idx_canal1] = MapTile::new(SEA_LEVEL);
        map.tiles[tile_idx_canal2] = MapTile::new(SEA_LEVEL);
        map.tiles[tile_idx_canal3] = MapTile::new(SEA_LEVEL);
        // lake
        let tile_idx_lake_center = map_idx(26, 19);
        let tile_idx_lake_up = map_idx(26, 18);
        let tile_idx_lake_down = map_idx(26, 20);
        let tile_idx_lake_right = map_idx(27, 19);
        map.tiles[tile_idx_lake_center] = MapTile::new(SEA_LEVEL);
        map.tiles[tile_idx_lake_up] = MapTile::new(SEA_LEVEL);
        map.tiles[tile_idx_lake_down] = MapTile::new(SEA_LEVEL);
        map.tiles[tile_idx_lake_right] = MapTile::new(SEA_LEVEL);
        // embankment
        let tile_idx_emb_canal_top1 = map_idx(23, 18);
        let tile_idx_emb_canal_top2 = map_idx(24, 18);
        let tile_idx_emb_canal_top3 = map_idx(25, 18);
        let tile_idx_emb_canal_bottom1 = map_idx(23, 20);
        let tile_idx_emb_canal_bottom2 = map_idx(24, 20);
        let tile_idx_emb_canal_bottom3 = map_idx(25, 20);
        map.tiles[tile_idx_emb_canal_top1] = MapTile::new(MAX_HEIGHT);
        map.tiles[tile_idx_emb_canal_top2] = MapTile::new(MAX_HEIGHT);
        map.tiles[tile_idx_emb_canal_top3] = MapTile::new(MAX_HEIGHT);
        map.tiles[tile_idx_emb_canal_bottom1] = MapTile::new(MAX_HEIGHT);
        map.tiles[tile_idx_emb_canal_bottom2] = MapTile::new(MAX_HEIGHT);
        map.tiles[tile_idx_emb_canal_bottom3] = MapTile::new(MAX_HEIGHT);

        let lake_center_tile = map.tiles[tile_idx_lake_center];
        assert_eq!(lake_center_tile.water, 0);  // no water

        // increase sea level to flood lake
        map.recalculate_sea_level(SEA_LEVEL+1);

        // now decrease it again
        map.recalculate_sea_level(SEA_LEVEL);

        // check that canal and lake have been emptied
        assert_eq!(map.tiles[tile_idx_canal1].water, 0);
        assert_eq!(map.tiles[tile_idx_canal2].water, 0);
        assert_eq!(map.tiles[tile_idx_canal3].water, 0);
        assert_eq!(map.tiles[tile_idx_lake_center].water, 0);
        assert_eq!(map.tiles[tile_idx_lake_up].water, 0);
        assert_eq!(map.tiles[tile_idx_lake_down].water, 0);
        assert_eq!(map.tiles[tile_idx_lake_right].water, 0);

    }
    #[test]
    fn decrease_height_inner_region_isolated_from_sea() {
        // create lake
        // lower height of neighboring tile
        // check if only expected tiles have been filled out
    }
    #[test]
    fn increase_height_inner_region_isolated_from_sea() {
        // create lake
        // flood
        // check if only expected tiles have been filled out
    }
    #[test]
    fn flood_inner_region_connecting_to_sea() {
        // create lake that will connect to sea if more water is added
        // flood
        // check the everything connecting to sea has been filled
    }

}