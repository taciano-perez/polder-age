use crate::prelude::*;

pub const SEA_BOTTOM: i32 = 1;
pub const SEA_LEVEL: i32 = 3;
pub const MAX_HEIGHT:i32 = 6;
const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Flatland,
    Water,
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MapTile {
    height: i32,
    water: i32,
}

impl MapTile {
    pub fn new(height: i32) -> Self {
    let water = if height <= SEA_LEVEL {SEA_LEVEL - height} else {0};
        Self {
            height,
            water,
        }
    }
    pub fn tile_type(&self)  -> TileType {
        if self.water > 0 { TileType::Water }
        else { TileType::Flatland }
    }
    pub fn increase_height(&mut self) {
        self.height = cmp::min(self.height+1, MAX_HEIGHT);
    }
    pub fn lower_height(&mut self) {
        self.height = cmp::max(self.height-1, SEA_BOTTOM);
    }
    pub fn add_water(&mut self, amount: i32) {
        self.water = cmp::min(self.water+amount, MAX_HEIGHT-self.height);
    }
    pub fn remove_water(&mut self, amount: i32) {
        let start = self.water;
        self.water = cmp::max(self.water-amount, 0);
        println!("Removed {} water, from {} to {}", amount, start, self.water);
    }
    fn water_level(&self) -> i32 {
        self.water + self.height
    }
    pub fn screen_glyph(&self) -> char {
        // let radix = 10;
        // match char::from_digit(self.water_level().wrapping_abs() as u32, radix) {
        //     Some(glyph) => { glyph },
        //     None => { 'N' },
        // }
       match self.tile_type() {
            TileType::Flatland => ' ',
            TileType::Water => '~',
       }
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
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum WaterEvent {
    Flood,
    Drain,
}

pub struct Map {
    pub tiles: Vec<MapTile>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y*SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![MapTile::new(0); NUM_TILES],
        }
    }
    pub fn increase_height(&mut self, x: i32, y: i32) {
        let idx = map_idx(x, y);
        self.tiles[idx].increase_height();
        if self.tiles[idx].water > 0 { self.trigger_event(x, y, WaterEvent::Drain, 1); }
    }
    pub fn lower_height(&mut self, x: i32, y: i32) {
        let idx = map_idx(x, y);
        self.tiles[idx].lower_height();
        self.flood_if_needed(x, y);
    }
    fn flood_if_needed(&mut self, x: i32, y: i32) {
        let neighbors = self.get_neighbors_water_level(x, y);
        let min = neighbors.iter().min();
        if let Some(Some(min)) = min {
            if let Some(water_level) = self.get_water_level(x, y) {
                if water_level < *min {
                    self.trigger_event(x, y, WaterEvent::Flood, min - water_level);
                }
            }
        }
    }
    fn get_neighbors_water_level(&mut self, x: i32, y: i32) -> Vec<Option<i32>> {
        vec![self.get_water_level(x+1, y),
        self.get_water_level(x-1, y),
        self.get_water_level(x, y+1),
        self.get_water_level(x, y-1)]
    }
    pub fn get_water_level(&mut self,x: i32, y: i32) -> Option<i32> {
        if !(0..SCREEN_WIDTH).contains(&x) { return None; };
        if !(0..SCREEN_HEIGHT).contains(&y) { return None; };
        let idx = map_idx(x, y);
        Some(self.tiles[idx].water_level())
    }
    /// Adds an amount of water to a tile, which spreads across the neighboring tiles.
    /// Based on the algorithm proposed here: https://stackoverflow.com/questions/60960372/flowing-water-in-byte-array-based-terrain
    pub fn trigger_event(&mut self, x:i32 , y: i32, event_type: WaterEvent, amount: i32) {
        let idx = map_idx(x, y);
        if event_type == WaterEvent::Flood && self.tiles[idx].height >= MAX_HEIGHT { return }
        if event_type == WaterEvent::Drain && self.tiles[idx].height <= SEA_BOTTOM { return }
        let mut visited = vec![vec![false; (SCREEN_HEIGHT+1) as usize]; (SCREEN_WIDTH+1) as usize];
        let mut to_be_visited = Vec::new();
        println!("Starting to {:?}...", event_type);
        match event_type {
            WaterEvent::Flood => { to_be_visited.push((x, y, self.tiles[idx].water_level()+1)); },
            WaterEvent::Drain => { to_be_visited.push((x, y, self.tiles[idx].water_level()-1)); },
        }
        while let Some((x, y, level)) = to_be_visited.pop() {
            self.bfs_update(Coordinate::new(x, y), event_type, amount, level, &mut visited, &mut to_be_visited);
        }
    }
    // breadth-first spread of water.
    fn bfs_update(&mut self, pos: Coordinate, event_type: WaterEvent, amount: i32, level: i32, visited: &mut [Vec<bool>], to_be_visited: &mut Vec<(i32, i32, i32)>) {
        let x = pos.x;
        let y = pos.y;
        let x_ = x as usize;
        let y_ = y as usize;
        let idx = map_idx(x, y);

        // check bounds
        if !(0..SCREEN_WIDTH).contains(&x) { return };
        if !(0..SCREEN_HEIGHT).contains(&y) { return };
        if visited[x_][y_] { 
            return;
         }

        // mark as visited
        visited[x_][y_] = true;

        // println!("Visiting {:?}", self.tiles[idx]);
        match event_type {
            WaterEvent::Flood => {
                if self.tiles[idx].height >= MAX_HEIGHT { return }
                // if the level isn't higher than the previous tile, flood it
                if self.tiles[idx].water_level() <= level {
                    if self.tiles[idx].water == 0 {
                        // needs 1 update and return
                        self.tiles[idx].add_water(1);
                        // println!("Increasing water amount by one, new level: {}", self.tiles[idx].water_level());
                        return;
                    }

                    self.tiles[idx].add_water(amount);
                    // println!("Increasing water amount by {}, new water: {}, new level: {}", amount, self.tiles[idx].water, self.tiles[idx].water_level());
                    // spread to neighboring tiles with lowest water level
                    let neighbors = self.get_neighbors_water_level(x, y);
                    let min = neighbors.iter().min();
                    if let Some(Some(min)) = min {
                        if let Some(tile_level) = neighbors[2] {
                            if tile_level == *min { to_be_visited.push((x, y+1, self.tiles[idx].water_level())); }
                        }
                        if let Some(tile_level) = neighbors[3] {
                            if tile_level == *min { to_be_visited.push((x, y-1, self.tiles[idx].water_level())); }
                        }
                        if let Some(tile_level) = neighbors[0] {
                            if tile_level == *min { to_be_visited.push((x+1, y, self.tiles[idx].water_level())); }
                        }
                        if let Some(tile_level) = neighbors[1] {
                            if tile_level == *min { to_be_visited.push((x-1, y, self.tiles[idx].water_level())); }
                        }
                    }
                }
            },
            WaterEvent::Drain => {
                if self.tiles[idx].height <= SEA_BOTTOM { return }
                // if there's water in the tile, drain it
                if self.tiles[idx].water > 0 {
                    // let start = self.tiles[idx].water_level();
                    self.tiles[idx].remove_water(amount);
                    // println!("lowering water level from {} to {}" , start, self.tiles[idx].water_level());
                    to_be_visited.push((x, y+1, self.tiles[idx].water_level()));
                    to_be_visited.push((x, y-1, self.tiles[idx].water_level()));
                    to_be_visited.push((x+1, y, self.tiles[idx].water_level()));
                    to_be_visited.push((x-1, y, self.tiles[idx].water_level()));
                }
            },
        }
    }
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                let tile = self.tiles[idx];
                match tile.tile_type() {
                    TileType::Flatland => {
                        ctx.set(x, y, tile.screen_color_fg(), tile.screen_color_bg(), to_cp437(tile.screen_glyph()));
                    }
                    TileType::Water => {
                        ctx.set(x, y, tile.screen_color_fg(), tile.screen_color_bg(), to_cp437(tile.screen_glyph()));
                    }
                }
            }
        }
    }
}