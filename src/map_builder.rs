use crate::prelude::*;

pub struct MapBuilder {
    pub map: Map,
}

impl MapBuilder {
    fn fill_with_random_land_tiles(&mut self, rng: &mut RandomNumberGenerator) {
        self.map.tiles.iter_mut()
            .for_each(|t| *t = MapTile::new(SEA_LEVEL + rng.range(0,MAX_HEIGHT-SEA_LEVEL+1)));
    }
    fn fill_rect(&mut self, rect: Rect, tile: MapTile) {
        rect.for_each(|p| {
            if p.x >= 0 && p.y >= 0 && p.x < SCREEN_WIDTH && p.y < SCREEN_HEIGHT {
                let idx = map_idx(p.x, p.y);
                self.map.tiles[idx] = tile;
            }
        })
    }
    fn build_random_map(&mut self, rng: &mut RandomNumberGenerator) {
        // land
        self.fill_with_random_land_tiles(rng);
        // sea
        let coast_row = rng.range(3, (SCREEN_HEIGHT / 2) - 5);
        self.fill_rect(Rect::with_size(0, 0, SCREEN_WIDTH, coast_row), MapTile::new(SEA_BOTTOM));
        // river
        let river_col = rng.range(6, (SCREEN_WIDTH / 2) - 6);
        let river_width = rng.range(1, 3);
        self.fill_rect(Rect::with_size(river_col, coast_row, river_width, SCREEN_HEIGHT-coast_row), MapTile::new(SEA_BOTTOM));
    }
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
        };
        mb.build_random_map(rng);
        mb
    }
}