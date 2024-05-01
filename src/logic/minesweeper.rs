pub struct minesweeper {
    width: u32,
    height: u32,
    mines: u32,

    // This is a 2D array of bools
    board: Vec<Vec<bool>>,
}
