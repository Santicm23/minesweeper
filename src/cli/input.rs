use inquire::Select;
use inquire::Text;
use regex::Regex;

use crate::config::constants::game::PlayingOptions;

pub fn select_game_option(previous: &PlayingOptions) -> PlayingOptions {
    let options = vec![
        PlayingOptions::Reveal,
        PlayingOptions::Flag,
        PlayingOptions::Quit,
    ];

    let cursor = options.iter().position(|x| x == previous).unwrap();

    Select::new("Choose an option", options)
        .with_starting_cursor(cursor)
        .prompt()
        .unwrap()
}

pub fn get_coords() -> Result<(u32, u32), &'static str> {
    let split;

    let text = Text::new("Enter the coordinates (x,y): ").prompt().unwrap();

    match Regex::new(r"(^\d+,\d+$)|(?<n>^$)").unwrap().captures(&text) {
        None => {
            return Err("Invalid coordinates, please enter in the format x,y");
        }
        Some(caps) => {
            if caps.name("n").is_some() {
                return Err("Exit");
            }
        }
    }

    split = text
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();
    Ok((split[0], split[1]))
}
