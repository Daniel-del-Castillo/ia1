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
    wall_percentage: usize,
}

impl FrontEnd {
    pub fn new(grid: Grid, wall_percentage: usize) -> Self {
        assert!(wall_percentage <= 100);
        FrontEnd {
            grid,
            state: State::Wall,
            wall_percentage,
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
