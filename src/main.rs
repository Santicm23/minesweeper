use std::env;

use minesweeper_lib::config::constants::args::Args;
use minesweeper_lib::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <height> <width> <mines>", args[0]);
        std::process::exit(1);
    }

    let width: u32 = args[1].parse().expect("Width must be a unsigned number");
    let height: u32 = args[2].parse().expect("Height must be a unsigned number");
    let mines: u32 = args[3].parse().expect("Mines must be a unsigned number");

    let args = Args::new(width, height, mines).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    run(args);
}
