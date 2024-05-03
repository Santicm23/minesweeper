use inquire::Select;
use inquire::Text;
use regex::Regex;

use crate::config::constants::game::PlayingOptions;

pub fn select_game_option() -> PlayingOptions {
    let options = vec![
        PlayingOptions::Reveal,
        PlayingOptions::Flag,
        PlayingOptions::Quit,
    ];

    Select::new("Choose an option", options).prompt().unwrap()
}

pub fn get_coords() -> (u32, u32) {
    let split;
    loop {
        let text = Text::new("Enter the coordinates (x, y): ")
            .prompt()
            .unwrap();

        match Regex::new(r"\d+,\d+").unwrap().captures(&text) {
            None => {
                println!("Invalid coordinates");
                continue;
            }
            _ => (),
        }

        split = text
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u32>>();
        break;
    }
    (split[0], split[1])
}
