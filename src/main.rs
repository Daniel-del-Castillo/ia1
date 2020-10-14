use clap::{App, Arg};
use crossterm::Result;
mod grid;
use grid::Grid;
mod frontend;
use frontend::FrontEnd;

fn main() -> Result<()> {
    let matches = App::new("ia1")
        .arg(
            Arg::with_name("rows")
                .short("m")
                .takes_value(true)
                .help("Set the number of initial rows"),
        )
        .arg(
            Arg::with_name("columns")
                .short("n")
                .takes_value(true)
                .help("Set the number of initial columns"),
        )
        .get_matches();
    let m = matches.value_of("rows").unwrap_or("10");
    let n = matches.value_of("columns").unwrap_or("10");
    let m = match m.parse() {
        Err(_) | Ok(0) => {
            eprintln!("The -m parameter must be a positive integer");
            std::process::exit(-1);
        }
        Ok(num) => num,
    };
    let n = match n.parse() {
        Err(_) | Ok(0) => {
            eprintln!("The -m parameter must be a positive integer");
            std::process::exit(-1);
        }
        Ok(num) => num,
    };
    let grid = Grid::new(m, n);
    let mut frontend = FrontEnd::new(grid);
    frontend.run()
}
