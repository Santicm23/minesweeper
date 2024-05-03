use std::fmt::Display;

pub enum BoardError {
    InvalidMove,
    MoveAlreadyPlayed,
    MoveAlreadyMarked,
    GameNotInitialized,
}

impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardError::InvalidMove => write!(f, "Invalid move"),
            BoardError::MoveAlreadyPlayed => write!(f, "Move already played"),
            BoardError::MoveAlreadyMarked => write!(f, "Move already marked"),
            BoardError::GameNotInitialized => write!(f, "Game not initialized"),
        }
    }
}
