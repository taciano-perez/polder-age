mod map;
mod map_builder;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use std::cmp;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    pub use crate::map::*;
    pub use crate::map_builder::*;
}

use prelude::*;

struct State {
    map: Map,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self { map: map_builder.map }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Space => { 
                    println!("Flooding...");
                    self.map.increase_water(0, 0, 1);
                },
                VirtualKeyCode::Return => { 
                    println!("Flooding...");
                    self.map.increase_water(1, 1, 1);
                }
                _ => {}
            }
        }
        ctx.cls();
        self.map.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
    .with_title("Age of Polders")
    .with_fps_cap(30.0)
    .build()?;

    main_loop(context, State::new())
}
