use std::fmt::{Display, Formatter};

use rand::Rng;

use crate::config::constants::errors::BoardError;

pub struct MinesWeeper {
    width: usize,
    height: usize,
    mines: usize,
    board: Vec<Vec<u8>>,
    masked_board: Vec<Vec<bool>>,
    marked_board: Vec<Vec<bool>>,
    total_marked: usize,
    initialized: bool,
    pub game_lost: bool,
    pub game_won: bool,
}

impl MinesWeeper {
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        Self {
            width,
            height,
            mines,
            board: vec![vec![0; width]; height],
            masked_board: vec![vec![true; width]; height],
            marked_board: vec![vec![false; width]; height],
            total_marked: 0,
            initialized: false,
            game_lost: false,
            game_won: false,
        }
    }

    fn init_board(&mut self, x: usize, y: usize) {
        self.populate_board(x, y);

        self.masked_board[y][x] = false;

        for i in 0..self.height {
            for j in 0..self.width {
                if self.board[i][j] == 9 {
                    continue;
                }

                self.board[i][j] = self.count_mines_surrounding(j, i);
            }
        }

        self.initialized = true;
    }

    fn populate_board(&mut self, x: usize, y: usize) {
        let mut rng = rand::thread_rng();

        let mut mines_to_place = self.mines;
        while mines_to_place > 0 {
            let i = rng.gen_range(0..self.width);
            let j = rng.gen_range(0..self.height);

            if self.board[j][i] == 9 || (i == x && j == y) {
                continue;
            }

            self.board[j][i] = 9;
            mines_to_place -= 1;
        }
    }

    fn count_mines_surrounding(&mut self, x: usize, y: usize) -> u8 {
        let mut mines = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let x = x as isize + i;
                let y = y as isize + j;

                if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
                    continue;
                }

                if self.board[y as usize][x as usize] == 9 {
                    mines += 1;
                }
            }
        }

        mines
    }

    pub fn play(&mut self, x: usize, y: usize) -> Result<(), BoardError> {
        if 1 > x || x > self.width || 1 > y || y > self.height {
            return Err(BoardError::InvalidMove);
        }

        let x = x - 1;
        let y = y - 1;

        if !self.masked_board[y][x] {
            return Err(BoardError::MoveAlreadyPlayed);
        }

        if self.marked_board[y][x] {
            return Err(BoardError::MoveAlreadyMarked);
        }

        if !self.initialized {
            self.init_board(x, y);
        }

        self.update_game_state(x, y);

        Ok(())
    }

    pub fn toggle_mark(&mut self, x: usize, y: usize) -> Result<(), BoardError> {
        if 1 > x || x > self.width || 1 > y || y > self.height {
            return Err(BoardError::InvalidMove);
        }

        let x = x - 1;
        let y = y - 1;

        if !self.initialized {
            return Err(BoardError::GameNotInitialized);
        }

        if x >= self.width || y >= self.height {
            return Err(BoardError::InvalidMove);
        }

        if !self.masked_board[y][x] {
            return Err(BoardError::MoveAlreadyPlayed);
        }

        match self.marked_board[y][x] {
            true => {
                self.marked_board[y][x] = false;
                self.total_marked -= 1;
            }
            false => {
                self.marked_board[y][x] = true;
                self.total_marked += 1;
            }
        }

        Ok(())
    }

    fn update_game_state(&mut self, x: usize, y: usize) {
        if self.board[y][x] == 9 {
            self.game_lost = true;
            self.unmask_board();
            return;
        }

        self.masked_board[y][x] = false;

        if self.has_won() {
            self.game_won = true;
            self.unmask_board();
            self.mark_all_mines();
        }
    }

    fn has_won(&self) -> bool {
        for i in 0..self.height {
            for j in 0..self.width {
                if !(self.masked_board[i][j] == (self.board[i][j] == 9)) {
                    return false;
                }
            }
        }

        true
    }

    fn unmask_board(&mut self) {
        self.masked_board = vec![vec![false; self.width]; self.height];
    }

    fn mark_all_mines(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.board[i][j] == 9 {
                    self.marked_board[i][j] = true;
                }
            }
        }
    }

    pub fn mines_left(&self) -> usize {
        self.mines - self.total_marked
    }

    pub fn is_game_over(&self) -> bool {
        self.game_lost || self.game_won
    }
}

impl Display for MinesWeeper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let delimiter: String = format!("+{}", "---+".repeat(self.height));

        writeln!(f, "{}", delimiter)?;

        for row in 0..self.height {
            for col in 0..self.width {
                let cell = self.board[row][col];

                let cell_str = if self.marked_board[row][col] {
                    "⚑".to_string()
                } else if self.masked_board[row][col] {
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
