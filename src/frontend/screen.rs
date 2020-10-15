use super::{FrontEnd, State};
use crossterm::{
    cursor::MoveTo,
    queue,
    style::Styler,
    terminal::{size, Clear, ClearType},
    Result,
};
use std::fmt::Write as fmt_write;
use std::io::{stdout, Write};

impl FrontEnd {
    pub fn draw_screen(&mut self) -> Result<()> {
        let mut buf = String::new();
        queue!(buf, Clear(ClearType::All))?;
        self.draw_grid(&mut buf)?;
        self.draw_buttons(&mut buf)?;
        self.draw_status_bar(&mut buf)?;
        write!(stdout(), "{}", buf)?;
        stdout().flush()?;
        Ok(())
    }

    fn draw_grid(&mut self, buf: &mut String) -> Result<()> {
        queue!(buf, MoveTo(0, 0))?;
        write!(buf, "{}", self.grid)?;
        Ok(())
    }

    fn draw_buttons(&mut self, buf: &mut String) -> Result<()> {
        queue!(buf, MoveTo(0, size()?.1 - 2))?;
        write!(
            buf,
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

    fn draw_status_bar(&mut self, buf: &mut String) -> Result<()> {
        write!(buf, "")?;
        Ok(())
    }
}
