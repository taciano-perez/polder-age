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
        let tile_type =
            if self.water > 0 { TileType::Water }
            else { TileType::Flatland };
        tile_type
    }
    pub fn screen_glyph(&self) -> char {
        // let RADIX = 10;
        // let result = match char::from_digit(self.height.wrapping_abs() as u32, RADIX) {
        //     Some(glyph) => { glyph },
        //     None => { 'N' },
        // };
        // result
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
    fn water_level(&self) -> i32 {
        self.water + self.height
    }
}

pub struct Map {
    pub tiles: Vec<MapTile>,
    counter: i32,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y*SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![MapTile::new(0); NUM_TILES],
            counter: 0,
        }
    }
    /// Adds an amount of water to a tile, which spreads across the neighboring tiles.
    /// Based on the algorithm proposed here: https://stackoverflow.com/questions/60960372/flowing-water-in-byte-array-based-terrain
    pub fn increase_water(&mut self, x:i32 , y: i32, amount: i32) {
        let idx = map_idx(x, y);
        if self.tiles[idx].height >= MAX_HEIGHT { return }
        let mut visited = vec![vec![false; (SCREEN_HEIGHT+1) as usize]; (SCREEN_WIDTH+1) as usize];
        let mut to_be_visited = Vec::new();
        to_be_visited.push((x, y, self.tiles[idx].water_level()+1));
        // println!("Starting to flood...");
        while let Some((x, y, level)) = to_be_visited.pop() {
            self.dfs_update(x, y, amount, level, &mut visited, &mut to_be_visited);
        }
        // println!("Flood visited {} tiles", self.counter);
    }
    // depth-first spread of water.
    fn dfs_update(&mut self, x:i32 , y: i32, amount: i32, level: i32, visited: &mut Vec<Vec<bool>>, to_be_visited: &mut Vec<(i32, i32, i32)>) {
        let x_ = x as usize;
        let y_ = y as usize;
        let idx = map_idx(x, y);

        // check bounds
        if x < 0 || x >= SCREEN_WIDTH { return };
        if y < 0 || y >= SCREEN_HEIGHT { return };
        if visited[x_][y_] == true { 
            return;
         }

        // mark as visited
        visited[x_][y_] = true;

        self.counter = self.counter + 1;
        // println!("Visiting {:?}", self.tiles[idx]);
        // println!("x: {}, y: {}, previous level: {}, water level: {}", x, y, level, self.tiles[idx].water_level());

        // MAX_HEIGHT cannot be flooded
        if self.tiles[idx].height >= MAX_HEIGHT { return }

        // if the level is lower than the previous tile, flood it
        if self.tiles[idx].water_level() <= level {
            if self.tiles[idx].water == 0 {
                // needs 1 update and return
                self.tiles[idx].water += 1;
                // println!("Increasing water amount by one, new level: {}", self.tiles[idx].water_level());
                return;
            }

            let max_water = MAX_HEIGHT - self.tiles[idx].height;
            self.tiles[idx].water = cmp::min(self.tiles[idx].water + amount, max_water);

            // 'recursively' call neighboring cells
            to_be_visited.push((x+1, y, self.tiles[idx].water_level()));
            to_be_visited.push((x-1, y, self.tiles[idx].water_level()));
            to_be_visited.push((x, y+1, self.tiles[idx].water_level()));
            to_be_visited.push((x, y-1, self.tiles[idx].water_level()));
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