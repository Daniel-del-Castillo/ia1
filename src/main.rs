use clap::{App, Arg, ArgMatches};
use crossterm::{terminal::size, Result};
mod grid;
use grid::Grid;
mod frontend;
use frontend::{FrontEnd, Heuristic};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let matches = get_args_matches();
    let grid = get_grid(&matches)?;
    let wall_percentage = get_wall_percentage(&matches);
    let heuristic = get_heuristic(&matches);
    let mut frontend = FrontEnd::new(grid, wall_percentage, heuristic);
    if matches.is_present("compat") {
        frontend.use_compatibility_mode();
    }
    frontend.run()
}

fn get_args_matches() -> ArgMatches<'static> {
    App::new("ia1")
        .args(&[
            Arg::with_name("m")
                .short("m")
                .long("rows")
                .takes_value(true)
                .help("Sets the number of initial rows"),
            Arg::with_name("n")
                .short("n")
                .long("columns")
                .takes_value(true)
                .help("Sets the number of initial columns"),
            Arg::with_name("wall_percentage")
                .short("r")
                .long("random")
                .takes_value(true)
                .help("Sets the percentage of walls in a random generated map"),
            Arg::with_name("euclidean")
                .long("euclidean")
                .conflicts_with("manhattan")
                .conflicts_with("chebyshev")
                .help("Uses euclidean distance as the heuristic function"),
            Arg::with_name("manhattan")
                .long("manhattan")
                .help("Uses manhattan distance as the heuristic function. This is the default"),
            Arg::with_name("chebyshev")
                .long("chebyshev")
                .help("Uses chebyshev distance as the heuristic function"),
            Arg::with_name("compat")
                .long("compat")
                .help("Uses a compatibility mode. Use this option if you can't see the car or the goal flag correctly"),
            Arg::with_name("file")
                .long("file")
                .short("f")
                .takes_value(true)
                .conflicts_with_all(&["m", "n"])
                .help("Reads a map from the specified file")
                .long_help(
                    "Reads a map from the specified file
                The file must have:
                -The number of rows in the first row
                -The number of columns in the second column
                -A representation of the map using:
                    -C. as the car
                    -G. as the goal
                    -X. as walls
                    -another character as empty cells",
                ),
        ])
        .get_matches()
}

fn get_grid(matches: &ArgMatches) -> Result<Grid> {
    if let Some(file) = matches.value_of("file") {
        get_grid_from_file(file)
    } else {
        get_grid_from_args(matches)
    }
}

fn get_grid_from_file(path: &str) -> Result<Grid> {
    let file = File::open(path)?;
    let mut buffer = BufReader::new(file).lines();
    let m = buffer.next().unwrap()?.parse::<usize>()?;
    let n = buffer.next().unwrap()?.parse::<usize>()?;
    check_valid_size(m, n)?;
    let mut grid = Grid::new(m, n);
    for i in 0..m {
        let row = buffer.next().unwrap()?;
        let mut row_chars = row.chars();
        for j in 0..n {
            let c = row_chars.next().unwrap();
            match c {
                'C' => grid.set_car(j, i),
                'G' => grid.set_goal(j, i),
                'X' => grid.set_wall(j, i),
                _ => {}
            }
        }
    }
    Ok(grid)
}

fn get_grid_from_args(matches: &ArgMatches) -> Result<Grid> {
    let (m, n) = get_grid_size(&matches);
    check_valid_size(m, n)?;
    Ok(Grid::new(m, n))
}

fn get_grid_size(matches: &ArgMatches) -> (usize, usize) {
    let m = matches.value_of("m").unwrap_or("10");
    let n = matches.value_of("n").unwrap_or("10");
    let m = match m.parse() {
        Err(_) | Ok(0) => {
            eprintln!("The -m parameter must be a positive integer");
            std::process::exit(-1);
        }
        Ok(num) => num,
    };
    let n = match n.parse() {
        Err(_) | Ok(0) => {
            eprintln!("The -n parameter must be a positive integer");
            std::process::exit(-1);
        }
        Ok(num) => num,
    };

    (m, n)
}

fn check_valid_size(m: usize, n: usize) -> Result<()> {
    let term_size = size()?;
    if m > term_size.1 as usize - 4 {
        eprintln!(
            "There isn't space in your terminal for a grid with a height of {}",
            m
        );
        eprintln!(
            "The maximum for the actual size of your terminal is {}",
            term_size.1 as usize - 4
        );
        std::process::exit(-1);
    } else if n > term_size.0 as usize / 2 - 2 {
        eprintln!(
            "There isn't space in your terminal for a grid with a width of {}",
            n
        );
        eprintln!(
            "The maximum for the actual size of your terminal is {}",
            term_size.0 as usize / 2 - 2
        );
        std::process::exit(-1);
    }
    Ok(())
}

fn get_wall_percentage(matches: &ArgMatches) -> usize {
    let wall_percentage = matches.value_of("wall_percentage").unwrap_or("15");
    let wall_percentage = match wall_percentage.parse() {
        Ok(num @ 0..=100) => num,
        _ => {
            eprintln!("The -r parameter must be a positive integer between 0 and 100");
            std::process::exit(-1);
        }
    };

    wall_percentage
}

fn get_heuristic(matches: &ArgMatches) -> Heuristic {
    if matches.is_present("euclidean") {
        return Heuristic::Euclidean;
    }
    if matches.is_present("chebyshev") {
        return Heuristic::Chebyshev;
    }
    Heuristic::Manhattan
}
