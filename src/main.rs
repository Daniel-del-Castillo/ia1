use crossterm::{cursor::Hide, execute, terminal::EnterAlternateScreen, Result};
use std::io::{stdout, Write};
mod grid;
use grid::Grid;
mod event_loop;
use event_loop::event_loop;

fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), Hide)?;
    let mut grid = Grid::new(1, 1);
    event_loop(&mut grid)
}
