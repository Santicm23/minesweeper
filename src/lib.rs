pub mod config;
pub mod logic;

use config::constants::args::Args;
use logic::minesweeper::MinesWeeper;

pub fn run(args: Args) {
    let minesweeper = MinesWeeper::new(args.width, args.height, args.mines);

    println!("{}", minesweeper)
}
