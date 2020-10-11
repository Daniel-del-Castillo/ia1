use crossterm::{
    cursor::Hide,
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
    Result,
};
use std::io::{stdout, Write};
mod grid;
use grid::Grid;
mod events;
use events::process_event;
mod screen;
use screen::draw_screen;

#[derive(PartialEq)]
pub enum State {
    Car,
    Goal,
    Wall,
}
fn main() -> Result<()> {
    execute!(stdout(), Hide, EnableMouseCapture, EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut grid = Grid::new(1, 1);
    event_loop(&mut grid)
}

fn event_loop(grid: &mut Grid) -> Result<()> {
    let mut state = State::Wall;
    loop {
        draw_screen(grid, &state)?;
        process_event(&mut state)?;
    }
}
