use crossterm::style::Colorize;
use std::fmt;
mod content;
use content::Content;

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

    pub fn m(&self) -> usize {
        self.grid.len()
    }

    pub fn n(&self) -> usize {
        self.grid[0].len()
    }

    pub fn set_wall(&mut self, x: usize, y: usize) {
        match &mut self.grid[y][x] {
            Content::Car => self.car = None,
            Content::Goal => self.goal = None,
            _ => {}
        }
        self.grid[y][x] = Content::Wall;
    }

    pub fn set_goal(&mut self, x: usize, y: usize) {
        match &mut self.grid[y][x] {
            Content::Car => self.car = None,
            Content::Goal => return,
            _ => {}
        }
        self.grid[y][x] = Content::Goal;
        if let Some(old_goal_pos) = &mut self.goal {
            self.grid[old_goal_pos.1][old_goal_pos.0] = Content::Empty;
        }
        self.goal = Some((x, y));
    }

    pub fn set_car(&mut self, x: usize, y: usize) {
        match &mut self.grid[y][x] {
            Content::Goal => self.goal = None,
            Content::Car => return,
            _ => {}
        }
        self.grid[y][x] = Content::Car;
        if let Some(old_car_pos) = &mut self.car {
            self.grid[old_car_pos.1][old_car_pos.0] = Content::Empty;
        }
        self.car = Some((x, y));
    }

    pub fn set_empty(&mut self, x: usize, y: usize) {
        match &mut self.grid[y][x] {
            Content::Goal => self.goal = None,
            Content::Car => self.car = None,
            _ => {}
        }
        self.grid[y][x] = Content::Empty;
    }
}
