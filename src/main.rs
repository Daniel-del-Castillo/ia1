use crossterm::{
    cursor::{Hide, MoveTo},
    execute, queue,
    terminal::{Clear, ClearType, EnterAlternateScreen},
    Result,
};
use std::fmt::Write as fmt_Write;
use std::io::{stdout, Write};
mod grid;
use grid::{Content, Grid};

fn main() -> Result<()> {
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), Hide)?;
    let grid = Grid {
        grid: vec![
            vec![Content::Wall, Content::Empty],
            vec![Content::Wall, Content::Car],
        ],
        m: 2,
        n: 2,
    };
    let mut buf = String::new();
    queue!(buf, Clear(ClearType::All))?;
    queue!(buf, MoveTo(0, 0))?;
    write!(buf, "{}", grid)?;
    write!(stdout(), "{}", buf)?;
    stdout().flush()?;
    loop {}
}
