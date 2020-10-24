use super::{Content, Grid};
use std::cmp::{max, min};
use std::collections::VecDeque;

impl Grid {
    pub fn clear_path(&mut self) {
        for cell in self.grid.iter_mut().map(|row| row.iter_mut()).flatten() {
            if let Content::Trace = cell {
                *cell = Content::Empty;
            }
        }
    }

    pub fn find_path(&mut self) -> bool {
        assert!(self.car.is_some() && self.goal.is_some());
        let car_pos = self.car.unwrap();
        let goal_pos = self.goal.unwrap();
        let mut queue = VecDeque::new();
        queue.push_back(car_pos);

        let mut predecessors = vec![vec![None; self.n()]; self.m()];

        let mut dist_map = vec![vec![std::usize::MAX; self.n()]; self.m()];
        dist_map[car_pos.1][car_pos.0] = 0;

        let mut guessed_dist_map = vec![vec![std::usize::MAX; self.n()]; self.m()];
        guessed_dist_map[car_pos.1][car_pos.0] = get_manhattan_dist(car_pos, goal_pos);

        while !queue.is_empty() {
            let current_pos = queue.pop_front().unwrap();
            if current_pos == goal_pos {
                self.draw_path(predecessors, car_pos, goal_pos);
                return true;
            }

            for neigh in self.get_neighbours(current_pos) {
                let dist = dist_map[current_pos.1][current_pos.0] + 1;
                if dist < dist_map[neigh.1][neigh.0] {
                    predecessors[neigh.1][neigh.0] = Some(current_pos);
                    dist_map[neigh.1][neigh.0] = dist;
                    // guessed_dist_map[neigh.1][neigh.0] =
                    //     dist_map[neigh.1][neigh.0] + get_manhattan_dist(neigh, goal_pos);
                    queue.push_back(neigh);
                }
            }
        }
        false
    }

    fn draw_path(
        &mut self,
        predecessors_map: Vec<Vec<Option<(usize, usize)>>>,
        car_pos: (usize, usize),
        goal_pos: (usize, usize),
    ) {
        let mut current = goal_pos;
        loop {
            current = predecessors_map[current.1][current.0].unwrap();
            if current == car_pos {
                break;
            }
            self.grid[current.1][current.0] = Content::Trace;
        }
    }

    fn get_neighbours(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighs = Vec::new();
        if pos.0 + 1 < self.n() {
            neighs.push((pos.0 + 1, pos.1));
        }
        if pos.0 != 0 {
            neighs.push((pos.0 - 1, pos.1));
        }
        if pos.1 + 1 < self.m() {
            neighs.push((pos.0, pos.1 + 1));
        }
        if pos.1 != 0 {
            neighs.push((pos.0, pos.1 - 1));
        }
        neighs
            .into_iter()
            .filter(|pos| {
                if let Content::Wall = self.grid[pos.1][pos.0] {
                    false
                } else {
                    true
                }
            })
            .collect()
    }
}

fn get_manhattan_dist(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    max(pos1.0, pos2.0) - min(pos1.0, pos2.0) + max(pos1.1, pos2.1) - min(pos1.1, pos2.1)
}
