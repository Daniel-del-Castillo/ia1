use super::{FrontEnd, State};
use crossterm::{
    cursor::Show,
    event::{read, DisableMouseCapture, Event, MouseButton, MouseEvent},
    execute,
    terminal::{disable_raw_mode, size, LeaveAlternateScreen},
    Result,
};
use std::cmp::{max, min};
use std::io::{stdout, Write};

const ROWS_MINUS_BUTTON_BEGIN: u16 = 0;
const ROWS_MINUS_BUTTON_END: u16 = 1;
const ROWS_PLUS_BUTTON_BEGIN: u16 = 6;
const ROWS_PLUS_BUTTON_END: u16 = 7;
const COLUMNS_MINUS_BUTTON_BEGIN: u16 = 9;
const COLUMNS_MINUS_BUTTON_END: u16 = 10;
const COLUMNS_PLUS_BUTTON_BEGIN: u16 = 18;
const COLUMNS_PLUS_BUTTON_END: u16 = 19;
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

impl FrontEnd {
    pub fn process_event(&mut self) -> Result<()> {
        let term_size = size()?;
        let buttons_y = term_size.1 - 2;
        match read()? {
            Event::Mouse(MouseEvent::Down(MouseButton::Left, x, y, ..)) => {
                if self.is_inside_grid((x as usize, y as usize)) {
                    self.set_cell((x as usize, y as usize));
                } else if y == buttons_y {
                    self.process_clicked_button(x, term_size);
                }
            }
            Event::Mouse(MouseEvent::Drag(MouseButton::Left, x, y, ..)) => {
                if self.is_inside_grid((x as usize, y as usize)) {
                    self.set_cell((x as usize, y as usize));
                }
            }
            _ => {}
        }
        Ok(())
    }

    //keep in mind that cell are two spaces wide
    fn is_inside_grid(&self, pos: (usize, usize)) -> bool {
        pos.0 <= self.grid.n() * 2 + 1 && pos.0 >= 2 && pos.1 <= self.grid.m() && pos.1 >= 1
    }

    fn set_cell(&mut self, pos: (usize, usize)) {
        match self.state {
            State::Car => self
                .grid
                .set_car(pos.0 as usize / 2 - 1, pos.1 as usize - 1),
            State::Goal => self
                .grid
                .set_goal(pos.0 as usize / 2 - 1, pos.1 as usize - 1),
            State::Wall => self
                .grid
                .set_wall(pos.0 as usize / 2 - 1, pos.1 as usize - 1),
            State::Remove => self
                .grid
                .set_empty(pos.0 as usize / 2 - 1, pos.1 as usize - 1),
        }
    }

    fn process_clicked_button(&mut self, x: u16, term_size: (u16, u16)) {
        if x >= ROWS_MINUS_BUTTON_BEGIN && x <= ROWS_MINUS_BUTTON_END {
            let desired_height = self.grid.m() - 1;
            self.grid
                .set_height(min(max(desired_height, 1), term_size.1 as usize - 4));
        } else if x >= ROWS_PLUS_BUTTON_BEGIN && x <= ROWS_PLUS_BUTTON_END {
            let desired_height = self.grid.m() + 1;
            self.grid
                .set_height(min(max(desired_height, 1), term_size.1 as usize - 4));
        } else if x >= COLUMNS_MINUS_BUTTON_BEGIN && x <= COLUMNS_MINUS_BUTTON_END {
            let desired_width = self.grid.n() - 1;
            self.grid
                .set_width(min(max(desired_width, 1), term_size.0 as usize / 2 - 2));
        } else if x >= COLUMNS_PLUS_BUTTON_BEGIN && x <= COLUMNS_PLUS_BUTTON_END {
            let desired_width = self.grid.n() + 1;
            self.grid
                .set_width(min(max(desired_width, 1), term_size.0 as usize / 2 - 2));
        } else if x >= CAR_BUTTON_BEGIN && x <= CAR_BUTTON_END {
            self.state = State::Car;
        } else if x >= GOAL_BUTTON_BEGIN && x <= GOAL_BUTTON_END {
            self.state = State::Goal;
        } else if x >= WALL_BUTTON_BEGIN && x <= WALL_BUTTON_END {
            self.state = State::Wall;
        } else if x >= REMOVE_BUTTON_BEGIN && x <= REMOVE_BUTTON_END {
            self.state = State::Remove;
        } else if x >= QUIT_BUTTON_BEGIN && x <= QUIT_BUTTON_END {
            quit();
        }
    }
}

fn quit() -> ! {
    disable_raw_mode().unwrap();
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture, Show).unwrap();
    std::process::exit(0);
}
