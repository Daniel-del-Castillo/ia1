use super::{FrontEnd, Heuristic};
use std::cmp::{max, min};

impl FrontEnd {
    pub(super) fn run_simulation(&mut self) {
        self.grid.clear_path();
        if !self.check_valid_state() {
            self.status_msg = String::from("A goal and a car must be in the grid");
            return;
        }
        if !self.grid.find_path(match self.heuristic {
            Heuristic::Euclidean => get_euclidean_dist,
            Heuristic::Manhattan => get_manhattan_dist,
        }) {
            self.status_msg = String::from("Couldn't find a path");
        } else {
            self.status_msg = String::from("Path found!");
        }
    }

    fn check_valid_state(&self) -> bool {
        self.grid.has_car() && self.grid.has_goal()
    }
}

fn get_manhattan_dist(pos1: (usize, usize), pos2: (usize, usize)) -> f32 {
    (max(pos1.0, pos2.0) - min(pos1.0, pos2.0) + max(pos1.1, pos2.1) - min(pos1.1, pos2.1)) as f32
}

fn get_euclidean_dist(pos1: (usize, usize), pos2: (usize, usize)) -> f32 {
    let pos1 = (pos1.0 as f32, pos1.1 as f32);
    let pos2 = (pos2.0 as f32, pos2.1 as f32);
    ((pos1.0 - pos2.0).exp2() + (pos1.1 - pos2.1).exp2()).sqrt()
}