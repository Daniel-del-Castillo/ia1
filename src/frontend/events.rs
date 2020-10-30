use super::{FrontEnd, Heuristic, State};
use crossterm::{
    cursor::Show,
    event::{poll, read, DisableMouseCapture, Event, MouseButton, MouseEvent},
    execute,
    terminal::{disable_raw_mode, size, LeaveAlternateScreen},
    Result,
};
use std::cmp::{max, min};
use std::io::{stdout, Write};
use std::time::Duration;

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
const CLEAR_BUTTON_BEGIN: u16 = 42;
const CLEAR_BUTTON_END: u16 = 46;
const RANDOM_BUTTON_BEGIN: u16 = 48;
const RANDOM_BUTTON_END: u16 = 53;
const HEURISTIC_BUTTON_BEGIN: u16 = 55;
const HEURISTIC_BUTTON_END: u16 = 63;
const RUN_BUTTON_BEGIN: u16 = 65;
const RUN_BUTTON_END: u16 = 67;
const QUIT_BUTTON_BEGIN: u16 = 69;
const QUIT_BUTTON_END: u16 = 72;

impl FrontEnd {
    pub(super) fn process_event(&mut self) -> Result<()> {
        let term_size = size()?;
        let buttons_y = term_size.1 - 2;
        match read()? {
            Event::Mouse(MouseEvent::Down(MouseButton::Left, x, y, ..)) => {
                self.status_msg.clear();
                if self.is_inside_grid((x as usize, y as usize)) {
                    self.set_cell((x as usize, y as usize));
                } else if y == buttons_y {
                    self.process_clicked_button(x, term_size)?;
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

    fn process_clicked_button(&mut self, x: u16, term_size: (u16, u16)) -> Result<()> {
        if x >= ROWS_MINUS_BUTTON_BEGIN && x <= ROWS_MINUS_BUTTON_END {
            self.change_height_while_clicked(-1, term_size.1 as usize)?;
        } else if x >= ROWS_PLUS_BUTTON_BEGIN && x <= ROWS_PLUS_BUTTON_END {
            self.change_height_while_clicked(1, term_size.1 as usize)?;
        } else if x >= COLUMNS_MINUS_BUTTON_BEGIN && x <= COLUMNS_MINUS_BUTTON_END {
            self.change_width_while_clicked(-1, term_size.0 as usize)?;
        } else if x >= COLUMNS_PLUS_BUTTON_BEGIN && x <= COLUMNS_PLUS_BUTTON_END {
            self.change_width_while_clicked(1, term_size.0 as usize)?;
        } else if x >= CAR_BUTTON_BEGIN && x <= CAR_BUTTON_END {
            self.state = State::Car;
        } else if x >= GOAL_BUTTON_BEGIN && x <= GOAL_BUTTON_END {
            self.state = State::Goal;
        } else if x >= WALL_BUTTON_BEGIN && x <= WALL_BUTTON_END {
            self.state = State::Wall;
        } else if x >= REMOVE_BUTTON_BEGIN && x <= REMOVE_BUTTON_END {
            self.state = State::Remove;
        } else if x >= CLEAR_BUTTON_BEGIN && x <= CLEAR_BUTTON_END {
            self.grid.clear();
        } else if x >= RANDOM_BUTTON_BEGIN && x <= RANDOM_BUTTON_END {
            self.grid.fill_random(self.wall_percentage);
        } else if x >= HEURISTIC_BUTTON_BEGIN && x <= HEURISTIC_BUTTON_END {
            self.change_heuristic();
        } else if x >= RUN_BUTTON_BEGIN && x <= RUN_BUTTON_END {
            self.run_simulation();
        } else if x >= QUIT_BUTTON_BEGIN && x <= QUIT_BUTTON_END {
            quit();
        }
        Ok(())
    }

    fn change_height_while_clicked(&mut self, change: isize, term_height: usize) -> Result<()> {
        loop {
            let desired_height = self.grid.m() as isize + change;
            self.grid
                .set_height(min(max(desired_height, 1) as usize, term_height - 4));
            self.draw_screen()?;
            if poll(Duration::from_millis(50))? {
                if let Event::Mouse(MouseEvent::Up(MouseButton::Left, ..)) = read()? {
                    break;
                }
            }
        }
        Ok(())
    }

    fn change_width_while_clicked(&mut self, change: isize, term_width: usize) -> Result<()> {
        loop {
            let desired_width = self.grid.n() as isize + change;
            self.grid
                .set_width(min(max(desired_width, 1) as usize, term_width / 2 - 2));
            self.draw_screen()?;
            if poll(Duration::from_millis(50))? {
                if let Event::Mouse(MouseEvent::Up(MouseButton::Left, ..)) = read()? {
                    break;
                }
            }
        }
        Ok(())
    }

    fn change_heuristic(&mut self) {
        self.heuristic = match self.heuristic {
            Heuristic::Euclidean => Heuristic::Manhattan,
            Heuristic::Manhattan => Heuristic::Euclidean,
        }
    }
}

fn quit() -> ! {
    disable_raw_mode().unwrap();
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture, Show).unwrap();
    std::process::exit(0);
}
