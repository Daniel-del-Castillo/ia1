use super::content::{Content, Direction};
use super::Grid;
use hashers::fx_hash::FxHasher;
use std::collections::{HashMap, VecDeque};
use std::hash::BuildHasherDefault;

#[derive(Copy, Clone)]
struct AStarNode {
    pos: (usize, usize),
    predecessor: Option<(usize, usize)>,
    dist: usize,
    guessed_dist: f32,
}

impl Default for AStarNode {
    fn default() -> Self {
        AStarNode {
            pos: (0, 0),
            predecessor: None,
            dist: usize::MAX,
            guessed_dist: f32::MAX,
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
        }
    }
}

pub struct PathResult {
    explored: usize,
    node_map: HashMap<(usize, usize), AStarNode, BuildHasherDefault<FxHasher>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Grid {
    pub fn find_path(
        &mut self,
        heuristic: fn((usize, usize), (usize, usize)) -> f32,
    ) -> Option<PathResult> {
        assert!(self.car.is_some() && self.goal.is_some());
        let car_pos = self.car.unwrap();
        let goal_pos = self.goal.unwrap();

        let mut node_map = HashMap::with_hasher(BuildHasherDefault::<FxHasher>::default());
        node_map.insert(
            car_pos,
            AStarNode {
                pos: car_pos,
                predecessor: None,
                dist: 0,
                guessed_dist: heuristic(car_pos, goal_pos),
            },
        );

        let mut priority_queue = VecDeque::new();
        priority_queue.push_front(car_pos);
        let mut iteration_count = 0;

        while !priority_queue.is_empty() {
            iteration_count += 1;
            let index = priority_queue
                .iter()
                .enumerate()
                .fold((0, f32::MAX), |acc, (index, node)| {
                    if node_map[node].guessed_dist < acc.1 {
                        (index, node_map[node].guessed_dist)
                    } else {
                        acc
                    }
                })
                .0;
            let current = priority_queue.remove(index).unwrap();
            let current = node_map[&current];
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
            let dist = node_map[&current.pos].dist + 1;
            for neigh_pos in self.get_neighbours(current.pos) {
                if dist < node_map.get(&neigh_pos).unwrap_or_default().dist {
                    let neigh_node = node_map.entry(neigh_pos).or_default();
                    neigh_node.pos = neigh_pos;
                    neigh_node.predecessor = Some(current.pos);
                    neigh_node.dist = dist;
                    neigh_node.guessed_dist = dist as f32 + heuristic(neigh_pos, goal_pos);
                    if !priority_queue.contains(&neigh_pos) {
                        priority_queue.push_front(neigh_pos);
                    }
                }
            }
        }
        None
    }

    fn draw_path(
        &mut self,
        node_map: &HashMap<(usize, usize), AStarNode, BuildHasherDefault<FxHasher>>,
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
