use grid::Grid;
use std::cmp::{max, min};
use std::time::Instant;
fn main() {
    println!(
        "{: <10} {: >12} {: >12} {: >12}",
        "Benchmark", "200x200", "100x100", "50x50"
    );
    println!(
        "{: <10} {: >10}μs {: >10}μs {: >10}μs",
        "Manhattan",
        get_average_duration(200, 200, 15, get_manhattan_dist, 500),
        get_average_duration(100, 100, 15, get_manhattan_dist, 500),
        get_average_duration(50, 50, 15, get_manhattan_dist, 500),
    );
    println!(
        "{: <10} {: >10}μs {: >10}μs {: >10}μs",
        "Euclidean",
        get_average_duration(200, 200, 15, get_euclidean_dist, 500),
        get_average_duration(100, 100, 15, get_euclidean_dist, 500),
        get_average_duration(50, 50, 15, get_euclidean_dist, 500),
    );
    println!(
        "{: <10} {: >10}μs {: >10}μs {: >10}μs",
        "Chebyshev",
        get_average_duration(200, 200, 15, get_chebyshev_dist, 500),
        get_average_duration(100, 100, 15, get_chebyshev_dist, 500),
        get_average_duration(50, 50, 15, get_chebyshev_dist, 500),
    );
}

fn get_average_duration(
    m: usize,
    n: usize,
    wall_percentage: usize,
    heuristic: fn((usize, usize), (usize, usize)) -> f32,
    repetitions: usize,
) -> u128 {
    let mut grid = Grid::new(m, n);
    let mut acc = 0;
    for _ in 0..repetitions {
        acc += loop {
            grid.fill_random(wall_percentage);
            let instant = Instant::now();
            let result = grid.find_path(heuristic);
            let duration = instant.elapsed();
            if let Some(_) = result {
                break duration.as_micros();
            }
        }
    }
    acc / repetitions as u128
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
