use crossterm::style::Colorize;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Content {
    Car,
    Goal,
    Wall,
    Empty,
    Trace(Direction),
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Content::Car => write!(f, "🚗"),
            Content::Goal => write!(f, "[]"),
            Content::Wall => write!(f, "{}", "██".red()),
            Content::Empty => write!(f, "  "),
            Content::Trace(dir) => match dir {
                Direction::Left => write!(f, "←←"),
                Direction::Up => write!(f, "↑↑"),
                Direction::Right => write!(f, "→→"),
                Direction::Down => write!(f, "↓↓"),
            },
        }
    }
}
