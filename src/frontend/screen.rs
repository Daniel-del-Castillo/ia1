use super::{FrontEnd, State};
use crossterm::{
    cursor::MoveTo,
    queue,
    style::Styler,
    terminal::{size, Clear, ClearType},
    Result,
};
use std::io::{stdout, Write};

impl FrontEnd {
    pub(super) fn draw_screen(&mut self) -> Result<()> {
        queue!(stdout(), Clear(ClearType::All))?;
        self.draw_grid()?;
        self.draw_buttons()?;
        self.draw_status_bar()?;
        stdout().flush()?;
        Ok(())
    }

    fn draw_grid(&mut self) -> Result<()> {
        queue!(stdout(), MoveTo(0, 0))?;
        write!(stdout(), "{}", self.grid)?;
        Ok(())
    }

    fn draw_buttons(&mut self) -> Result<()> {
        queue!(stdout(), MoveTo(0, size()?.1 - 2))?;
        write!(
            stdout(),
            "{0}Rows{1} {0}Columns{1} {2} {3} {4} {5} {6} {7} {8} {9}\n\r",
            "--".negative(),
            "++".negative(),
            if self.state == State::Car {
                "Car".bold()
            } else {
                "Car".negative()
            },
            if self.state == State::Goal {
                "Goal".bold()
            } else {
                "Goal".negative()
            },
            if self.state == State::Wall {
                "Wall".bold()
            } else {
                "Wall".negative()
            },
            if self.state == State::Remove {
                "Remove".bold()
            } else {
                "Remove".negative()
            },
            "Clear".negative(),
            "Random".negative(),
            "Run".negative(),
            "Quit".negative()
        )?;
        Ok(())
    }

    fn draw_status_bar(&mut self) -> Result<()> {
        write!(stdout(), "{}", self.status_msg)?;
        Ok(())
    }
}
