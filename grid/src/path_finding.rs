use super::content::{Content, Direction};
use super::Grid;
use fxhash::FxHashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone)]
struct AStarNode {
    pos: (usize, usize),
    predecessor: Option<(usize, usize)>,
    dist: usize,
    guessed_dist: f32,
    depth: usize,
}

impl Default for AStarNode {
    fn default() -> Self {
        AStarNode {
            pos: (0, 0),
            predecessor: None,
            dist: usize::MAX,
            guessed_dist: f32::MAX,
            depth: 0,
        }
    }
}

impl Default for &AStarNode {
    fn default() -> Self {
        &AStarNode {
            pos: (0, 0),
            predecessor: None,
            dist: usize::MAX,
            guessed_dist: f32::MAX,
            depth: 0,
        }
    }
}

impl Eq for AStarNode {}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.guessed_dist == other.guessed_dist && self.depth == other.depth
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        //this will panic if guessed_dist is NaN. That shouldn't happen and the floating point
        //is needed to use heuristics like the euclidean distance
        other.guessed_dist.partial_cmp(&self.guessed_dist).unwrap()
    }
}

impl PartialOrd for AStarNode {
    //They get compared in inversed order
    //so an AStarNode will have more priority if its guessed distance is smaller
    //if equal the node with the higher depth will have more priority
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match other.guessed_dist.partial_cmp(&self.guessed_dist) {
            Some(Ordering::Equal) => {
                if other.depth < self.depth {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                }
            }
            x => x,
        }
    }
}

pub struct PathResult {
    explored: usize,
    node_map: FxHashMap<(usize, usize), AStarNode>,
    start: (usize, usize),
    end: (usize, usize),
}

impl PathResult {
    pub fn get_path_length(self) -> usize {
        let mut current = self.end;
        let mut lenght = 0;
        loop {
            current = self.node_map[&current].predecessor.unwrap();
            if current == self.start {
                break lenght;
            }
            lenght += 1;
        }
    }

    pub fn get_n_explored(&self) -> usize {
        self.explored
    }
}

impl Grid {
    pub fn find_path(
        &mut self,
        heuristic: fn((usize, usize), (usize, usize)) -> f32,
    ) -> Option<PathResult> {
        assert!(self.car.is_some() && self.goal.is_some());
        let car_pos = self.car.unwrap();
        let goal_pos = self.goal.unwrap();

        let mut node_map = FxHashMap::default();
        //reserve space for twice the space needed for the expected length upfront to avoid reallocations
        node_map.reserve(heuristic(car_pos, goal_pos) as usize * 2);
        node_map.insert(
            car_pos,
            AStarNode {
                pos: car_pos,
                predecessor: None,
                dist: 0,
                guessed_dist: heuristic(car_pos, goal_pos),
                depth: 0,
            },
        );

        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(node_map[&car_pos]);
        let mut iteration_count = 0;

        while !priority_queue.is_empty() {
            iteration_count += 1;
            let current = priority_queue.pop().unwrap();
            let current = node_map[&current.pos];
            if current.pos == goal_pos {
                self.draw_path(&node_map, car_pos, goal_pos);
                return Some(PathResult {
                    explored: iteration_count,
                    node_map,
                    start: car_pos,
                    end: goal_pos,
                });
            }
            if current.pos != car_pos {
                self.grid[current.pos.1][current.pos.0] = Content::Explored;
            }
            let dist = current.dist + 1;
            for neigh_pos in self.get_neighbours(current.pos) {
                if dist < node_map.get(&neigh_pos).unwrap_or_default().dist {
                    let neigh_node = node_map.entry(neigh_pos).or_default();
                    neigh_node.pos = neigh_pos;
                    neigh_node.predecessor = Some(current.pos);
                    neigh_node.dist = dist;
                    neigh_node.guessed_dist = dist as f32 + heuristic(neigh_pos, goal_pos);
                    neigh_node.depth = current.depth + 1;
                    priority_queue.push(neigh_node.clone());
                }
            }
        }
        None
    }

    fn draw_path(
        &mut self,
        node_map: &FxHashMap<(usize, usize), AStarNode>,
        start: (usize, usize),
        end: (usize, usize),
    ) {
        let mut current = end;
        loop {
            let prev = current;
            current = node_map[&current].predecessor.unwrap();
            if current == start {
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
