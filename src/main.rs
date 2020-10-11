use crossterm::{
    cursor::{Hide, MoveTo},
    execute, queue,
    terminal::{Clear, ClearType, EnterAlternateScreen},
    Result,
};
use std::fmt::Write as fmt_Write;
use std::io::{stdout, Write};
mod grid;
use grid::Grid;

fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), Hide)?;
    let mut grid = Grid::new(2, 2);
    grid.set_wall(0, 0).ok();
    grid.set_wall(0, 1).ok();
    grid.set_goal(1, 0).ok();
    grid.set_car(1, 1).ok();
    let mut buf = String::new();
    queue!(buf, Clear(ClearType::All))?;
    queue!(buf, MoveTo(0, 0))?;
    write!(buf, "{}", grid)?;
    write!(stdout(), "{}", buf)?;
    stdout().flush()?;
    loop {}
}
