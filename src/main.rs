use clap::{App, Arg, ArgMatches};
use crossterm::{terminal::size, Result};
mod grid;
use grid::Grid;
mod frontend;
use frontend::FrontEnd;

fn main() -> Result<()> {
    let (m, n) = parse_args();
    check_valid_size(m, n)?;
    let grid = Grid::new(m, n);
    let mut frontend = FrontEnd::new(grid);
    frontend.run()
}

fn parse_args() -> (usize, usize) {
    let matches = declare_args();
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

fn declare_args() -> ArgMatches<'static> {
    App::new("ia1")
        .arg(
            Arg::with_name("m")
                .short("m")
                .long("rows")
                .takes_value(true)
                .help("Set the number of initial rows"),
        )
        .arg(
            Arg::with_name("n")
                .short("n")
                .long("columns")
                .takes_value(true)
                .help("Set the number of initial columns"),
        )
        .get_matches()
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
