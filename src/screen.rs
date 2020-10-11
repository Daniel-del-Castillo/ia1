use crate::Grid;
use crate::State;
use crossterm::{
    cursor::MoveTo,
    queue,
    style::Styler,
    terminal::{size, Clear, ClearType},
    Result,
};
use std::fmt::Write as fmt_write;
use std::io::{stdout, Write};

pub fn draw_screen(grid: &Grid, state: &State) -> Result<()> {
    let mut buf = String::new();
    queue!(buf, Clear(ClearType::All))?;
    draw_grid(&mut buf, grid)?;
    draw_buttons(&mut buf, state)?;
    draw_status_bar(&mut buf)?;
    write!(stdout(), "{}", buf)?;
    stdout().flush()?;
    Ok(())
}

fn draw_grid(buf: &mut String, grid: &Grid) -> Result<()> {
    queue!(buf, MoveTo(0, 0))?;
    write!(buf, "{}", grid)?;
    Ok(())
}

fn draw_buttons(buf: &mut String, state: &State) -> Result<()> {
    queue!(buf, MoveTo(0, size()?.1 - 2))?;
    write!(
        buf,
        "{0}Rows{1} {0}Columns{1} {2} {3} {4} {5} {6}\n\r",
        "--".negative(),
        "++".negative(),
        if *state == State::Car {
            "Car".bold()
        } else {
            "Car".negative()
        },
        if *state == State::Goal {
            "Goal".bold()
        } else {
            "Goal".negative()
        },
        if *state == State::Wall {
            "Wall".bold()
        } else {
            "Wall".negative()
        },
        "Run".negative(),
        "Quit".negative()
    )?;
    Ok(())
}

fn draw_status_bar(buf: &mut String) -> Result<()> {
    write!(buf, "")?;
    Ok(())
}
