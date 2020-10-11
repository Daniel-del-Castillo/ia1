use crate::{Grid, State};
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
const REMOVE_BUTTON_BEGIN: u16 = 35;
const REMOVE_BUTTON_END: u16 = 40;
const QUIT_BUTTON_BEGIN: u16 = 45;
const QUIT_BUTTON_END: u16 = 48;

pub fn process_event(grid: &mut Grid, state: &mut State) -> Result<()> {
    let buttons_y = size()?.1 - 2;
    if let Event::Mouse(MouseEvent::Down(MouseButton::Left, x, y, ..)) = read()? {
        //If it is inside the border walls of the grid. Keep in mind that walls take two spaces wide
        if is_inside(grid, x, y) {
            match state {
                State::Car => grid.set_car(x as usize / 2 - 1, y as usize - 1),
                State::Goal => grid.set_goal(x as usize / 2 - 1, y as usize - 1),
                State::Wall => grid.set_wall(x as usize / 2 - 1, y as usize - 1),
                State::Remove => grid.set_empty(x as usize / 2 - 1, y as usize - 1),
            }
        } else if y != buttons_y {
            return Ok(());
        } else if x >= CAR_BUTTON_BEGIN && x <= CAR_BUTTON_END {
            *state = State::Car;
        } else if x >= GOAL_BUTTON_BEGIN && x <= GOAL_BUTTON_END {
            *state = State::Goal;
        } else if x >= WALL_BUTTON_BEGIN && x <= WALL_BUTTON_END {
            *state = State::Wall;
        } else if x >= REMOVE_BUTTON_BEGIN && x <= REMOVE_BUTTON_END {
            *state = State::Remove;
        } else if x >= QUIT_BUTTON_BEGIN && x <= QUIT_BUTTON_END {
            quit();
        }
    }
    Ok(())
}

fn is_inside(grid: &Grid, x: u16, y: u16) -> bool {
    x <= grid.n() as u16 * 2 + 1 && x >= 2 && y <= grid.m() as u16 && y >= 1
}

fn quit() -> ! {
    disable_raw_mode().unwrap();
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture, Show).unwrap();
    std::process::exit(0);
}
