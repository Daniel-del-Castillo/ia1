use crossterm::{
    cursor::Hide,
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
    Result,
};
use grid::Grid;
use std::io::{stdout, Write};
mod events;
mod run_simulation;
mod screen;

#[derive(PartialEq)]
enum State {
    Car,
    Goal,
    Wall,
    Remove,
}

#[derive(PartialEq)]
pub enum Heuristic {
    Euclidean,
    Manhattan,
    Chebyshev,
}

pub struct FrontEnd {
    grid: Grid,
    state: State,
    wall_percentage: usize,
    heuristic: Heuristic,
    status_msg: String,
    compatibility_mode: bool,
}

impl FrontEnd {
    pub fn new(grid: Grid, wall_percentage: usize, heuristic: Heuristic) -> Self {
        assert!(wall_percentage <= 100);
        FrontEnd {
            grid,
            state: State::Wall,
            wall_percentage,
            heuristic,
            status_msg: String::new(),
            compatibility_mode: false,
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

    pub fn use_compatibility_mode(&mut self) {
        self.compatibility_mode = true;
    }
}
