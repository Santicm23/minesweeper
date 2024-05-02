mod cli;
pub mod config;
pub mod logic;

use std::io::stdin;

use config::constants::args::Args;
use logic::minesweeper::MinesWeeper;

pub fn run(args: Args) {
    let minesweeper = MinesWeeper::new(args.width, args.height, args.mines);

    while !minesweeper.is_game_over() {
        print!("{}", minesweeper);
        println!("Enter your move (x, y):");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        println!("your input: {}", input);
    }
}
