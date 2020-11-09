use crossterm::style::Colorize;
use std::fmt;

#[derive(Clone, Copy)]
pub enum Content {
    Car,
    Goal,
    Wall,
    Empty,
    Trace(Direction),
    Explored,
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
            Content::Car => write!(f, "{}", "🚗".on_black()),
            Content::Goal => write!(f, "{}", "🏁".on_black()),
            Content::Wall => write!(f, "{}", "  ".on_red()),
            Content::Empty => write!(f, "{}", "  ".on_black()),
            Content::Explored => write!(f, "{}", "  ".on_yellow()),
            Content::Trace(dir) => match dir {
                Direction::Left => write!(f, "{}", "←←".on_black()),
                Direction::Up => write!(f, "{}", "↑↑".on_black()),
                Direction::Right => write!(f, "{}", "→→".on_black()),
                Direction::Down => write!(f, "{}", "↓↓".on_black()),
            },
        }
    }
}
