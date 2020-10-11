use crossterm::style::Colorize;
use std::fmt;
mod content;
use content::Content;
mod error;
use error::GridError;

pub struct Grid {
    grid: Vec<Vec<Content>>,
    goal: Option<(usize, usize)>,
    car: Option<(usize, usize)>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.grid[0].len() + 2 {
            write!(f, "{}", "██".dark_blue())?;
        }
        write!(f, "\n\r")?;
        for row in self.grid.iter() {
            write!(f, "{}", "██".dark_blue())?;
            for cell in row.iter() {
                write!(f, "{}", cell)?
            }
            write!(f, "{}", "██\n\r".dark_blue())?;
        }
        for _ in 0..self.grid[0].len() + 2 {
            write!(f, "{}", "██".dark_blue())?;
        }
        write!(f, "\n\r")?;
        Ok(())
    }
}

impl Grid {
    pub fn new(m: usize, n: usize) -> Self {
        assert!(m != 0 || n != 0 || m * n >= 2);
        Grid {
            grid: vec![vec![Content::Empty; n]; m],
            goal: None,
            car: None,
        }
    }

    pub fn set_wall(&mut self, x: usize, y: usize) -> Result<(), GridError> {
        match &mut self.grid[y][x] {
            Content::Car => Err(GridError::OverwriteCar),
            Content::Goal => Err(GridError::OverwriteGoal),
            content => {
                *content = Content::Wall;
                Ok(())
            }
        }
    }

    pub fn set_goal(&mut self, x: usize, y: usize) -> Result<(), GridError> {
        match &mut self.grid[y][x] {
            Content::Car => return Err(GridError::OverwriteCar),
            Content::Goal => return Ok(()),
            content => *content = Content::Goal,
        }
        if let Some(old_goal_pos) = &mut self.goal {
            self.grid[old_goal_pos.1][old_goal_pos.0] = Content::Empty;
        }
        self.goal = Some((x, y));
        Ok(())
    }

    pub fn set_car(&mut self, x: usize, y: usize) -> Result<(), GridError> {
        match &mut self.grid[y][x] {
            Content::Car => return Ok(()),
            Content::Goal => return Err(GridError::OverwriteGoal),
            content => *content = Content::Car,
        }
        if let Some(old_car_pos) = &mut self.car {
            self.grid[old_car_pos.1][old_car_pos.0] = Content::Empty;
        }
        self.goal = Some((x, y));
        Ok(())
    }
}
