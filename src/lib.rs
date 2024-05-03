mod cli;
pub mod config;
pub mod logic;

use clearscreen;

use cli::input::{get_coords, select_game_option};
use config::constants::args::Args;
use config::constants::game::PlayingOptions;
use logic::minesweeper::MinesWeeper;

pub fn run(args: Args) {
    let mut minesweeper = MinesWeeper::new(args.width, args.height, args.mines);

    while !minesweeper.is_game_over() {
        clearscreen::clear().unwrap();

        print!("{}", minesweeper);

        let input = select_game_option();

        handle_input(input, &mut minesweeper);
    }
}

fn handle_input(input: PlayingOptions, minesweeper: &mut MinesWeeper) {
    if let PlayingOptions::Quit = input {
        println!("Quitting game...");
        std::process::exit(0);
    }

    let (x, y) = get_coords();

    match input {
        PlayingOptions::Reveal => match minesweeper.play(x, y) {
            Ok(_) => (),
            Err(_) => println!("error"),
        },
        PlayingOptions::Flag => match minesweeper.toggle_mark(x, y) {
            Ok(_) => (),
            Err(_) => println!("error"),
        },
        _ => (),
    }
}
