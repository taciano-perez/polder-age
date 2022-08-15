mod map;
mod map_builder;
mod state;
mod command;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use bracket_terminal::prelude::*;
    pub use std::cmp;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;

    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::state::*;
    pub use crate::command::*;
}

use prelude::*;

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::F => { 
                    self.active_command = Command::Flood;
                },
                VirtualKeyCode::D => { 
                    self.active_command = Command::Drain;
                }
                VirtualKeyCode::R => { 
                    self.active_command = Command::RaiseHeight;
                }
                VirtualKeyCode::L => { 
                    self.active_command = Command::LowerHeight;
                }
                VirtualKeyCode::Return => { 
                    self.active_command = Command::RestartGame;
                }
                VirtualKeyCode::Space => { 
                    execute_command(self, self.active_command);
                }
                _ => {}
            }
        }
        ctx.cls();
        self.map.render(ctx);

        // TEST
        let mut input = INPUT.lock();
        // let mouse_pixels = input.mouse_pixel_pos();
        // ctx.print(
        //     1,
        //     1,
        //     &format!(
        //         "Mouse pixel position: {}, {}",
        //         mouse_pixels.0, mouse_pixels.1
        //     ),
        // );
        let mouse_tile = input.mouse_tile(0);
        // ctx.print(
        //     1,
        //     2,
        //     &format!("Mouse tile position: {}, {}", mouse_tile.x, mouse_tile.y),
        // );
        ctx.print(1, 3, &format!("Active Command: {:?}", self.active_command));
        ctx.print(1, 4, &format!("Selected tile x:{}, y: {}", self.selected_tile.x, self.selected_tile.y));

        for (i, btn) in input.mouse_button_pressed_set().iter().enumerate() {
            ctx.print(1, 5 + i as i32, &format!("Mouse Button {} is pressed", btn));
            self.selected_tile = Coordinate::new(mouse_tile.x, mouse_tile.y);
        }

        for (i, key) in input.scan_code_pressed_set().iter().enumerate() {
            ctx.print(50, 5 + i as i32, &format!("Key code: {}", key));
        }

        for (i, key) in input.key_pressed_set().iter().enumerate() {
            ctx.print(50, 25 + i as i32, &format!("Key code: {:?}", key));
        }

        input.for_each_message(|event| {
            bracket_terminal::console::log(&format!("{:#?}", event));
            if event == BEvent::CloseRequested {
                ctx.quitting = true;
            }
        });
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
    .with_title("Age of Polders")
    .with_fps_cap(30.0)
    .build()?;

    main_loop(context, State::new())
}
