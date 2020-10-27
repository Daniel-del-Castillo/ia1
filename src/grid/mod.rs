use crossterm::style::Colorize;
use std::fmt;
mod content;
mod path_finding;
use content::Content;
use rand::{thread_rng, Rng};

pub struct Grid {
    grid: Vec<Vec<Content>>,
    goal: Option<(usize, usize)>,
    car: Option<(usize, usize)>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.grid[0].len() + 2 {
            write!(f, "{}", "  ".on_dark_blue())?;
        }
        write!(f, "\n\r")?;
        for row in self.grid.iter() {
            write!(f, "{}", "  ".on_dark_blue())?;
            for cell in row.iter() {
                write!(f, "{}", cell)?
            }
            write!(f, "{}", "  \n\r".on_dark_blue())?;
        }
        for _ in 0..self.grid[0].len() + 2 {
            write!(f, "{}", "  ".on_dark_blue())?;
        }
        write!(f, "\n\r")?;
        Ok(())
    }
}

impl Grid {
    pub fn new(m: usize, n: usize) -> Self {
        assert!(m != 0 && n != 0);
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

    pub fn has_goal(&self) -> bool {
        match self.goal {
            None => false,
            Some(_) => true,
        }
    }

    pub fn has_car(&self) -> bool {
        match self.car {
            None => false,
            Some(_) => true,
        }
    }

    pub fn set_width(&mut self, n: usize) {
        assert!(n != 0);
        let width = self.grid[0].len();
        if n == width {
            return;
        } else if n < width {
            self.grid.iter_mut().for_each(|row| row.truncate(n));
            self.check_car_valididy();
            self.check_goal_valididy();
        } else {
            self.grid
                .iter_mut()
                .for_each(|row| (0..n - width).for_each(|_| row.push(Content::Empty)));
        }
    }

    pub fn set_height(&mut self, m: usize) {
        assert!(m != 0);
        let height = self.grid.len();
        if m == height {
            return;
        } else if m < height {
            self.grid.truncate(m);
            self.check_car_valididy();
            self.check_goal_valididy();
        } else {
            let width = self.grid[0].len();
            (0..m - height).for_each(|_| self.grid.push(vec![Content::Empty; width]));
        }
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

    pub fn clear(&mut self) {
        self.grid = vec![vec![Content::Empty; self.grid[0].len()]; self.grid.len()];
    }

    pub fn fill_random(&mut self, wall_percentage: usize) {
        assert!(wall_percentage <= 100);
        self.car = None;
        self.goal = None;
        self.fill_random_walls(wall_percentage);
        let car_pos = self.get_random_pos();
        self.set_car(car_pos.0, car_pos.1);
        //if the grid only has one cell we won't place the goal
        if self.m() * self.n() != 1 {
            let goal_pos = loop {
                let pos = self.get_random_pos();
                if pos != car_pos {
                    break pos;
                }
            };
            self.set_goal(goal_pos.0, goal_pos.1);
        }
    }

    fn fill_random_walls(&mut self, wall_percentage: usize) {
        let mut rng = thread_rng();
        for content in self.grid.iter_mut().map(|i| i.iter_mut()).flatten() {
            if rng.gen_range(1, 101) <= wall_percentage {
                *content = Content::Wall;
            } else {
                *content = Content::Empty;
            }
        }
    }

    fn get_random_pos(&mut self) -> (usize, usize) {
        let n_cells = self.m() * self.n();
        let pos = thread_rng().gen_range(0, n_cells);
        let y = pos / self.n();
        let x = pos % self.n();
        (x, y)
    }

    fn check_car_valididy(&mut self) {
        if let Some(pos) = self.car {
            if pos.0 >= self.n() || pos.1 >= self.m() {
                self.car = None;
            }
        }
    }

    fn check_goal_valididy(&mut self) {
        if let Some(pos) = self.goal {
            if pos.0 >= self.n() || pos.1 >= self.m() {
                self.goal = None;
            }
        }
    }
}
