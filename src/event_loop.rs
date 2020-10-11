use crate::Grid;
use crossterm::{
    cursor::MoveTo,
    queue,
    style::Styler,
    terminal::{size, Clear, ClearType},
    Result,
};
use std::fmt::Write as fmt_write;
use std::io::{stdout, Write};

pub fn event_loop(grid: &mut Grid) -> Result<()> {
    loop {
        draw_screen(grid)?;
    }
}

fn draw_screen(grid: &Grid) -> Result<()> {
    let mut buf = String::new();
    queue!(buf, Clear(ClearType::All))?;
    queue!(buf, MoveTo(0, 0))?;
    write!(buf, "{}", grid)?;
    draw_buttons(&mut buf)?;
    write!(stdout(), "{}", buf)?;
    stdout().flush()?;
    Ok(())
}

fn draw_buttons(buf: &mut String) -> Result<()> {
    queue!(buf, MoveTo(0, size()?.1 - 2))?;
    write!(
        buf,
        "{0}Rows{1} {0}Columns{1} {2} {3} {4} {5} {6}\n",
        "--".negative(),
        "++".negative(),
        "Car".negative(),
        "Goal".negative(),
        "Wall".negative(),
        "Run".negative(),
        "Quit".negative()
    )?;
    Ok(())
}
