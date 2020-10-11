use crossterm::style::Colorize;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Content {
    Car,
    Goal,
    Wall,
    Empty,
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Content::Car => write!(f, "🚗"),
            Content::Goal => write!(f, "[]"),
            Content::Wall => write!(f, "{}", "██".red()),
            Content::Empty => write!(f, "  "),
        }
    }
}
