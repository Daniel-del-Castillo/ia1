use crate::Grid;
mod events;
mod screen;
use crossterm::Result;
#[derive(PartialEq)]
enum State {
    Car,
    Goal,
    Wall,
    Remove,
}

pub struct FrontEnd {
    grid: Grid,
    state: State,
}

impl FrontEnd {
    pub fn new(grid: Grid) -> Self {
        FrontEnd {
            grid,
            state: State::Wall,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            self.draw_screen()?;
            self.process_event()?;
        }
    }
}
