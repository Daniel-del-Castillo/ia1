use super::{Content, Grid};
use std::cmp::{max, min, Ordering};
use std::collections::BinaryHeap;

#[derive(Copy, Clone)]
struct AStarNode {
    pos: (usize, usize),
    predecessor: Option<(usize, usize)>,
    dist: usize,
    guessed_dist: f32,
}

impl Eq for AStarNode {}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.guessed_dist == other.guessed_dist
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        //this will panic if guessed_dist is NaN. That shouldn't happen and the floating point
        //is needed to use heuristics like the euclidean distance
        self.guessed_dist.partial_cmp(&other.guessed_dist).unwrap()
    }
}

impl PartialOrd for AStarNode {
    //The values get flipped so an AStarNode will have more priority if its guessed distance is smaller
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.guessed_dist.partial_cmp(&other.guessed_dist) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Greater) => Some(Ordering::Less),
            x => x,
        }
    }
}

impl AStarNode {
    pub fn new(pos: (usize, usize)) -> Self {
        AStarNode {
            pos,
            predecessor: None,
            dist: usize::MAX,
            guessed_dist: f32::MAX,
        }
    }
}

impl Grid {
    pub fn find_path(&mut self) -> bool {
        assert!(self.car.is_some() && self.goal.is_some());
        let car_pos = self.car.unwrap();
        let goal_pos = self.goal.unwrap();

        let mut node_grid = Vec::new();
        for i in 0..self.m() {
            node_grid.push(Vec::new());
            for j in 0..self.n() {
                node_grid[i].push(AStarNode::new((j, i)));
            }
        }

        node_grid[car_pos.1][car_pos.0].dist = 0;
        node_grid[car_pos.1][car_pos.0].dist = get_manhattan_dist(car_pos, goal_pos);

        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(node_grid[car_pos.1][car_pos.0]);

        while !priority_queue.is_empty() {
            let current = priority_queue.pop().unwrap();
            if current.pos == goal_pos {
                self.draw_path(node_grid, car_pos, goal_pos);
                return true;
            }
            for neigh in self.get_neighbours(current.pos) {
                let dist = node_grid[current.pos.1][current.pos.0].dist + 1;
                if dist < node_grid[neigh.1][neigh.0].dist {
                    node_grid[neigh.1][neigh.0].predecessor = Some(current.pos);
                    node_grid[neigh.1][neigh.0].dist = dist;
                    node_grid[neigh.1][neigh.0].guessed_dist = node_grid[neigh.1][neigh.0].dist
                        as f32
                        + get_manhattan_dist(neigh, goal_pos) as f32;
                    priority_queue.push(node_grid[neigh.1][neigh.0]);
                }
            }
        }
        false
    }

    fn draw_path(
        &mut self,
        node_grid: Vec<Vec<AStarNode>>,
        car_pos: (usize, usize),
        goal_pos: (usize, usize),
    ) {
        let mut current = goal_pos;
        loop {
            current = node_grid[current.1][current.0].predecessor.unwrap();
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

    pub fn clear_path(&mut self) {
        for cell in self.grid.iter_mut().map(|row| row.iter_mut()).flatten() {
            if let Content::Trace = cell {
                *cell = Content::Empty;
            }
        }
    }
}

fn get_manhattan_dist(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    max(pos1.0, pos2.0) - min(pos1.0, pos2.0) + max(pos1.1, pos2.1) - min(pos1.1, pos2.1)
}
