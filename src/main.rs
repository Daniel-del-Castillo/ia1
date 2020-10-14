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
mod frontend;
use frontend::FrontEnd;

fn main() -> Result<()> {
    execute!(stdout(), Hide, EnableMouseCapture, EnterAlternateScreen)?;
    enable_raw_mode()?;
    let grid = Grid::new(10, 10);
    let mut frontend = FrontEnd::new(grid);
    frontend.run()
}
