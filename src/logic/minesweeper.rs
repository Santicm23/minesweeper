use std::fmt::{Display, Formatter};

use rand::Rng;

use crate::config::constants::errors::BoardError;

pub struct MinesWeeper {
    width: u32,
    height: u32,
    mines: u32,
    board: Vec<Vec<u8>>,
    masked_board: Vec<Vec<bool>>,
    marked_board: Vec<Vec<bool>>,
    total_marked: u32,
    initialized: bool,
    pub game_lost: bool,
    pub game_won: bool,
}

impl MinesWeeper {
    pub fn new(width: u32, height: u32, mines: u32) -> Self {
        Self {
            width,
            height,
            mines,
            board: vec![vec![0; width as usize]; height as usize],
            masked_board: vec![vec![true; width as usize]; height as usize],
            marked_board: vec![vec![false; width as usize]; height as usize],
            total_marked: 0,
            initialized: false,
            game_lost: false,
            game_won: false,
        }
    }

    fn init_board(&mut self, x: u32, y: u32) {
        self.populate_board(x, y);

        self.masked_board[y as usize][x as usize] = false;

        for i in 0..self.height {
            for j in 0..self.width {
                if self.board[i as usize][j as usize] == 9 {
                    continue;
                }

                self.board[i as usize][j as usize] = self.count_mines_surrounding(j, i);
            }
        }

        self.initialized = true;
    }

    fn populate_board(&mut self, x: u32, y: u32) {
        let mut rng = rand::thread_rng();

        let mut mines_to_place = self.mines;
        while mines_to_place > 0 {
            let i = rng.gen_range(0..self.width);
            let j = rng.gen_range(0..self.height);

            if self.board[j as usize][i as usize] == 9 || (i == x && j == y) {
                continue;
            }

            self.board[j as usize][i as usize] = 9;
            mines_to_place -= 1;
        }
    }

    fn count_mines_surrounding(&mut self, x: u32, y: u32) -> u8 {
        let mut mines = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let x = x as i32 + i;
                let y = y as i32 + j;

                if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
                    continue;
                }

                if self.board[y as usize][x as usize] == 9 {
                    mines += 1;
                }
            }
        }

        mines
    }

    pub fn play(&mut self, x: u32, y: u32) -> Result<(), BoardError> {
        if x >= self.width || y >= self.height {
            return Err(BoardError::InvalidMove);
        }

        if !self.masked_board[y as usize][x as usize] {
            return Err(BoardError::MoveAlreadyPlayed);
        }

        if self.marked_board[y as usize][x as usize] {
            return Err(BoardError::MoveAlreadyMarked);
        }

        if !self.initialized {
            self.init_board(x, y);
        }

        self.update_game_state(x, y);

        Ok(())
    }

    pub fn toggle_mark(&mut self, x: u32, y: u32) -> Result<(), BoardError> {
        if !self.initialized {
            return Err(BoardError::GameNotInitialized);
        }

        if x >= self.width || y >= self.height {
            return Err(BoardError::InvalidMove);
        }

        if !self.masked_board[y as usize][x as usize] {
            return Err(BoardError::MoveAlreadyPlayed);
        }

        match self.marked_board[y as usize][x as usize] {
            true => {
                self.marked_board[y as usize][x as usize] = false;
                self.total_marked -= 1;
            }
            false => {
                self.marked_board[y as usize][x as usize] = true;
                self.total_marked += 1;
            }
        }

        Ok(())
    }

    fn update_game_state(&mut self, x: u32, y: u32) {
        if self.board[y as usize][x as usize] == 9 {
            self.game_lost = true;
            self.unmask_board();
            return;
        }

        self.masked_board[y as usize][x as usize] = false;

        if self.has_won() {
            self.game_won = true;
            self.unmask_board();
            self.mark_all_mines();
        }
    }

    fn has_won(&self) -> bool {
        for i in 0..self.height {
            for j in 0..self.width {
                if !(self.masked_board[i as usize][j as usize]
                    == (self.board[i as usize][j as usize] == 9))
                {
                    return false;
                }
            }
        }

        true
    }

    fn unmask_board(&mut self) {
        self.masked_board = vec![vec![false; self.width as usize]; self.height as usize];
    }

    fn mark_all_mines(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.board[i as usize][j as usize] == 9 {
                    self.marked_board[i as usize][j as usize] = true;
                }
            }
        }
    }

    pub fn mines_left(&self) -> u32 {
        self.mines - self.total_marked
    }

    pub fn is_game_over(&self) -> bool {
        self.game_lost || self.game_won
    }
}

impl Display for MinesWeeper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let delimiter: String = format!("+{}", "---+".repeat(self.height as usize));

        writeln!(f, "{}", delimiter)?;

        for row in 0..self.height {
            for col in 0..self.width {
                let cell = self.board[row as usize][col as usize];

                let cell_str = if self.marked_board[row as usize][col as usize] {
                    "⚑".to_string()
                } else if self.masked_board[row as usize][col as usize] {
                    " ".to_string()
                } else {
                    match cell {
                        9 => "✴".to_string(),
                        _ => format!("{}", cell),
                    }
                };

                write!(f, "| {} ", cell_str)?;
            }

            writeln!(f, "|")?;
            writeln!(f, "{}", delimiter)?;
        }

        Ok(())
    }
}
