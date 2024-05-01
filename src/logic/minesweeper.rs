use std::fmt::{Display, Formatter};

use rand::Rng;

use crate::logic::errors::BoardError;

pub struct MinesWeeper {
    width: u32,
    height: u32,
    mines: u32,
    board: Vec<Vec<u32>>,
    mines_board: Vec<Vec<bool>>,
    marked: Vec<Vec<bool>>,
    total_marked: u32,
}

impl MinesWeeper {
    pub fn new(width: u32, height: u32, mines: u32) -> Self {
        let board = vec![vec![9; width as usize]; height as usize];

        let mut mines_to_place = mines;
        let mut mines_board = vec![vec![false; width as usize]; height as usize];
        let mut rng = rand::thread_rng();

        while mines_to_place > 0 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            if mines_board[y as usize][x as usize] {
                continue;
            }

            mines_board[y as usize][x as usize] = true;
            mines_to_place -= 1;
        }

        Self {
            width,
            height,
            mines,
            board,
            mines_board,
            marked: vec![vec![false; width as usize]; height as usize],
            total_marked: 0,
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

                if self.mines_board[y as usize][x as usize] {
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

        if self.board[y as usize][x as usize] != 0 {
            return Err(BoardError::MoveAlreadyPlayed);
        }

        if self.mines_board[y as usize][x as usize] {
            return Err(BoardError::GameOver);
        }

        self.board[y as usize][x as usize] = self.count_mines_surrounding(x, y) as u32;

        Ok(())
    }

    pub fn toggle_mark(&mut self, x: u32, y: u32) -> Result<(), BoardError> {
        if x >= self.width || y >= self.height {
            return Err(BoardError::InvalidMove);
        }

        if self.board[y as usize][x as usize] != 0 {
            return Err(BoardError::MoveAlreadyPlayed);
        }

        match self.marked[y as usize][x as usize] {
            true => {
                self.marked[y as usize][x as usize] = true;
                self.total_marked += 1;
            }
            false => {
                self.marked[y as usize][x as usize] = false;
                self.total_marked -= 1;
            }
        }

        Ok(())
    }

    pub fn mines_left(&self) -> u32 {
        self.mines - self.total_marked
    }
}

impl Display for MinesWeeper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut delimiter = String::from("+");
        for _ in 0..self.height {
            delimiter.push_str("---+");
        }

        writeln!(f, "{}", delimiter)?;
        for row in self.board.iter() {
            write!(f, "|")?;
            for cell in row.iter() {
                let cell = match cell {
                    9 => " ".to_string(),
                    _ => format!("{}", cell),
                };
                write!(f, " {} |", cell)?;
            }
            writeln!(f, "\n{}", delimiter)?;
        }

        Ok(())
    }
}
