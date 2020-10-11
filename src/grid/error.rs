use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum GridError {
    OverwriteCar,
    OverwriteGoal,
}

impl Error for GridError {}

impl fmt::Display for GridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tried to overwrite the {}",
            match self {
                GridError::OverwriteCar => "Car",
                GridError::OverwriteGoal => "Goal",
            }
        )
    }
}
