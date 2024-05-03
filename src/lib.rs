mod cli;
pub mod config;
pub mod logic;

use std::process;

use clearscreen;

use cli::{get_coords, select_game_option};
use config::constants::Args;
use config::constants::PlayingOptions;
use logic::MinesWeeper;

pub fn run(args: Args) {
    let mut minesweeper = MinesWeeper::new(args.width, args.height, args.mines);

    while !minesweeper.is_game_over() {
        clearscreen::clear().unwrap();

        print!("{}", minesweeper);

        let input = select_game_option();

        handle_input(input, &mut minesweeper);
    }

    clearscreen::clear().unwrap();

    print!("{}", minesweeper);

    if minesweeper.game_won {
        println!("Congratulations! You won!");
    } else {
        println!("Game over! You lost!");
    }
}

fn handle_input(input: PlayingOptions, minesweeper: &mut MinesWeeper) {
    if let PlayingOptions::Quit = input {
        println!("Quitting game...");
        std::process::exit(0);
    }

    loop {
        let (x, y) = match get_coords() {
            Ok(coords) => coords,
            Err("Exit") => break,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };

        let res = match input {
            PlayingOptions::Reveal => minesweeper.play(x, y),
            PlayingOptions::Flag => minesweeper.toggle_mark(x, y),
            _ => process::exit(1),
        };

        match res {
            Err(err) => println!("Error: {}", err),
            _ => break,
        }
    }
}
