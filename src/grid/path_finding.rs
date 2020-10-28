use super::content::{Content, Direction};
use super::Grid;
use std::collections::VecDeque;

#[derive(Copy, Clone)]
struct AStarNode {
    pos: (usize, usize),
    predecessor: Option<(usize, usize)>,
    dist: usize,
    guessed_dist: f32,
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
    pub fn find_path(
        &mut self,
        heuristic: fn((usize, usize), (usize, usize)) -> f32,
    ) -> Option<usize> {
        assert!(self.car.is_some() && self.goal.is_some());
        let car_pos = self.car.unwrap();
        let goal_pos = self.goal.unwrap();

        let mut node_grid = self.create_node_grid();
        node_grid[car_pos.1][car_pos.0].dist = 0;
        node_grid[car_pos.1][car_pos.0].guessed_dist = heuristic(car_pos, goal_pos);

        let mut priority_queue = VecDeque::new();
        priority_queue.push_front(node_grid[car_pos.1][car_pos.0]);
        let mut iteration_count = 0;

        while !priority_queue.is_empty() {
            iteration_count += 1;
            let index = priority_queue
                .iter()
                .enumerate()
                .fold((0, f32::MAX), |acc, (index, node)| {
                    if node.guessed_dist < acc.1 {
                        (index, node.guessed_dist)
                    } else {
                        acc
                    }
                })
                .0;
            let current = priority_queue.remove(index).unwrap();
            if current.pos == goal_pos {
                self.draw_path(node_grid, car_pos, goal_pos);
                return Some(iteration_count);
            }
            if current.pos != car_pos {
                self.grid[current.pos.1][current.pos.0] = Content::Explored;
            }
            let dist = node_grid[current.pos.1][current.pos.0].dist + 1;
            for neigh in self.get_neighbours(current.pos) {
                if dist < node_grid[neigh.1][neigh.0].dist {
                    node_grid[neigh.1][neigh.0].predecessor = Some(current.pos);
                    node_grid[neigh.1][neigh.0].dist = dist;
                    node_grid[neigh.1][neigh.0].guessed_dist =
                        node_grid[neigh.1][neigh.0].dist as f32 + heuristic(neigh, goal_pos);
                    priority_queue.push_front(node_grid[neigh.1][neigh.0]);
                }
            }
        }
        None
    }

    fn create_node_grid(&self) -> Vec<Vec<AStarNode>> {
        let mut node_grid = Vec::new();
        for i in 0..self.m() {
            node_grid.push(Vec::new());
            for j in 0..self.n() {
                node_grid[i].push(AStarNode::new((j, i)));
            }
        }
        node_grid
    }

    fn draw_path(
        &mut self,
        node_grid: Vec<Vec<AStarNode>>,
        car_pos: (usize, usize),
        goal_pos: (usize, usize),
    ) {
        let mut current = goal_pos;
        loop {
            let prev = current;
            current = node_grid[current.1][current.0].predecessor.unwrap();
            if current == car_pos {
                break;
            }
            self.grid[current.1][current.0] = match current {
                (x, y) if x == prev.0 + 1 && y == prev.1 => Content::Trace(Direction::Left),
                (x, y) if x + 1 == prev.0 && y == prev.1 => Content::Trace(Direction::Right),
                (x, y) if x == prev.0 && y == prev.1 + 1 => Content::Trace(Direction::Up),
                (x, y) if x == prev.0 && y + 1 == prev.1 => Content::Trace(Direction::Down),
                _ => unreachable!("Corrupted predecessors table"),
            };
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
            if let Content::Trace(_) | Content::Explored = cell {
                *cell = Content::Empty;
            }
        }
    }
}
