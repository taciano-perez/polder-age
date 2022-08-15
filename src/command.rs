use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    Flood,
    Drain,
    RaiseHeight,
    LowerHeight,
    RestartGame,
}

pub fn execute_command(state: &mut State, command: Command) {
    match command {
        Command::Flood => {
            println!("Flooding...");
            state.map.trigger_event(state.selected_tile.x, state.selected_tile.y, WaterEvent::Flood, 1);
        },
        Command::Drain => {
            println!("Draining...");
            state.map.trigger_event(state.selected_tile.x, state.selected_tile.y, WaterEvent::Drain, 1);
        },
        Command::RaiseHeight => {
            println!("Increasing height...");
            state.map.increase_height(state.selected_tile.x, state.selected_tile.y);
        }
        Command::LowerHeight => {
            println!("Decreasing height...");
            state.map.lower_height(state.selected_tile.x, state.selected_tile.y);
        }
        Command::RestartGame => {
            println!("Restarting game...");
            *state = State::new();
        }
        // _ => {}
    }
}