use crate::prelude::*;

pub const SEA_LEVEL: i32 = 0;
const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;
const RADIX: u32 = 10;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Flatland,
    Water,
}

#[derive(Copy, Clone, PartialEq)]
pub struct MapTile {
    height: i32,
}

impl MapTile {
    pub fn new(height: i32) -> Self {
        Self { height }
    }
    pub fn tile_type(&self)  -> TileType {
        let tile_type =
            if self.height <= SEA_LEVEL { TileType::Water }
            else { TileType::Flatland };
        tile_type
    }
    pub fn screen_glyph(&self) -> char {
        char::from_digit(self.height.wrapping_abs() as u32, RADIX).unwrap()
    }
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
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                let tile = self.tiles[idx];
                match tile.tile_type() {
                    TileType::Flatland => {
                        ctx.set(x, y, GREEN, DARK_GREEN, to_cp437(tile.screen_glyph()));
                    }
                    TileType::Water => {
                        ctx.set(x, y, WHITE, BLUE, to_cp437(tile.screen_glyph()));
                    }
                }
            }
        }
    }
}