use super::{FrontEnd, Heuristic};
use std::cmp::{max, min};
use std::time::Instant;

impl FrontEnd {
    pub(super) fn run_simulation(&mut self) {
        self.grid.clear_path();
        if !self.check_valid_state() {
            self.status_msg = String::from("A goal and a car must be in the grid");
            return;
        }
        let heuristic_fn = match self.heuristic {
            Heuristic::Euclidean => get_euclidean_dist,
            Heuristic::Manhattan => get_manhattan_dist,
            Heuristic::Chebyshev => get_chebyshev_dist,
        };
        let time = Instant::now();
        let path_result = self.grid.find_path(heuristic_fn);
        let time_used = time.elapsed();
        match path_result {
            None => self.status_msg = String::from("Couldn't find a path"),
            Some(path_result) => {
                self.status_msg =
                    format!(
                    "Path with length {1} found! {0} cells were explored. Approximate time used: {2} μs",
                    path_result.get_n_explored(), path_result.get_path_length(), time_used.as_micros()
                );
            }
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
    ((pos1.0 - pos2.0).powi(2) + (pos1.1 - pos2.1).powi(2)).sqrt()
}

fn get_chebyshev_dist(pos1: (usize, usize), pos2: (usize, usize)) -> f32 {
    max(
        max(pos1.0, pos2.0) - min(pos1.0, pos2.0),
        max(pos1.1, pos2.1) - min(pos1.1, pos2.1),
    ) as f32
}
