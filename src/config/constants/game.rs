use std::fmt::Display;

pub enum PlayingOptions {
    Reveal,
    Flag,
    Quit,
}

impl Display for PlayingOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayingOptions::Reveal => write!(f, "Reveal a cell"),
            PlayingOptions::Flag => write!(f, "Flag a cell"),
            PlayingOptions::Quit => write!(f, "Quit game"),
        }
    }
}
