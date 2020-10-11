use crate::State;
use crossterm::{
    cursor::Show,
    event::{read, DisableMouseCapture, Event, MouseButton, MouseEvent},
    execute,
    terminal::{disable_raw_mode, size, LeaveAlternateScreen},
    Result,
};
use std::io::{stdout, Write};

const CAR_BUTTON_BEGIN: u16 = 21;
const CAR_BUTTON_END: u16 = 23;
const GOAL_BUTTON_BEGIN: u16 = 25;
const GOAL_BUTTON_END: u16 = 28;
const WALL_BUTTON_BEGIN: u16 = 30;
const WALL_BUTTON_END: u16 = 33;
const QUIT_BUTTON_BEGIN: u16 = 39;
const QUIT_BUTTON_END: u16 = 42;

pub fn process_event(state: &mut State) -> Result<()> {
    let buttons_y = size()?.1 - 2;
    if let Event::Mouse(MouseEvent::Down(MouseButton::Left, x, y, ..)) = read()? {
        if y != buttons_y {
            return Ok(());
        }
        if x >= CAR_BUTTON_BEGIN && x <= CAR_BUTTON_END {
            *state = State::Car;
        } else if x >= GOAL_BUTTON_BEGIN && x <= GOAL_BUTTON_END {
            *state = State::Goal;
        } else if x >= WALL_BUTTON_BEGIN && x <= WALL_BUTTON_END {
            *state = State::Wall;
        } else if x >= QUIT_BUTTON_BEGIN && x <= QUIT_BUTTON_END {
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
