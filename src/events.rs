use crossterm::{
    cursor::Show,
    event::{read, DisableMouseCapture, Event, MouseButton, MouseEvent},
    execute,
    terminal::{disable_raw_mode, size, LeaveAlternateScreen},
    Result,
};
use std::io::{stdout, Write};

const QUIT_BUTTON_BEGIN: u16 = 39;
const QUIT_BUTTON_END: u16 = 42;

pub fn process_event() -> Result<()> {
    let buttons_y = size()?.1 - 2;
    if let Event::Mouse(MouseEvent::Down(MouseButton::Left, x, y, ..)) = read()? {
        if y != buttons_y {
            return Ok(());
        }
        if x >= QUIT_BUTTON_BEGIN && x <= QUIT_BUTTON_END {
            quit();
        }
    }
    Ok(())
}

fn quit() -> ! {
    disable_raw_mode().unwrap();
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture, Show).unwrap();
    std::process::exit(0);
}
