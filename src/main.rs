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

fn main() -> Result<()> {
    execute!(stdout(), Hide, EnableMouseCapture, EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut grid = Grid::new(1, 1);
    event_loop(&mut grid)
}

fn event_loop(grid: &mut Grid) -> Result<()> {
    loop {
        draw_screen(grid)?;
        process_event()?;
    }
}
