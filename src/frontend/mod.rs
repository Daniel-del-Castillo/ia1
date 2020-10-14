use crate::Grid;
mod events;
mod screen;
use crossterm::{
    cursor::Hide,
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
    Result,
};
use std::io::{stdout, Write};

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
        execute!(stdout(), Hide, EnableMouseCapture, EnterAlternateScreen)?;
        enable_raw_mode()?;
        loop {
            self.draw_screen()?;
            self.process_event()?;
        }
    }
}
