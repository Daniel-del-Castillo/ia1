use std::fmt;

pub enum Content {
    Car,
    Wall,
    Empty,
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Content::Car => write!(f, "0"),
            Content::Wall => write!(f, "█"),
            Content::Empty => write!(f, " "),
        }
    }
}
pub struct Grid {
    pub grid: Vec<Vec<Content>>,
    pub m: usize, //rows
    pub n: usize, //columns
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.n + 2 {
            write!(f, "█")?;
        }
        write!(f, "\n")?;
        for row in self.grid.iter() {
            write!(f, "█")?;
            for cell in row.iter() {
                write!(f, "{}", cell)?
            }
            write!(f, "█\n")?;
        }
        for _ in 0..self.n + 2 {
            write!(f, "█")?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}
