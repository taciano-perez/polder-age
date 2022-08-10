use crate::prelude::*;

pub struct MapBuilder {
    pub map: Map,
}

impl MapBuilder {
    fn fill(&mut self, tile: MapTile) {
        self.map.tiles.iter_mut()
            .for_each(|t| *t = tile);
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
        self.fill(MapTile::new(SEA_LEVEL+1));
        // sea
        let coast_row = rng.range(3, (SCREEN_HEIGHT / 2) - 3);
        self.fill_rect(Rect::with_size(0, 0, SCREEN_WIDTH, coast_row), MapTile::new(SEA_LEVEL));
        // river
        let river_col = rng.range(6, (SCREEN_WIDTH / 2) - 6);
        let river_width = rng.range(1, 3);
        self.fill_rect(Rect::with_size(river_col, coast_row, river_width, SCREEN_HEIGHT-coast_row), MapTile::new(SEA_LEVEL));
    }
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
        };
        mb.build_random_map(rng);
        mb
    }
}