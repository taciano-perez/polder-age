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
        if self.height > SEA_BOTTOM {
            self.water = cmp::min(self.water+amount, MAX_HEIGHT-self.height);
        }
        // self.water += amount;
        // println!("Added {} water, from {} to {}", amount, start, self.water);
    }
pub fn remove_water(&mut self, amount: i32) {
        // let start = self.water;
        if self.height > SEA_BOTTOM {
            self.water = cmp::max(self.water-amount, 0);
        }
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
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y*SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new(river_source: Coordinate) -> Self {
        Self {
            tiles: vec![MapTile::new(0); NUM_TILES],
            river_source,
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