pub struct Args {
    pub width: usize,
    pub height: usize,
    pub mines: usize,
}

impl Args {
    pub fn new(width: usize, height: usize, mines: usize) -> Result<Args, String> {
        if width < 1 {
            return Err("Width must be greater than 0".to_string());
        }
        if height < 1 {
            return Err("Height must be greater than 0".to_string());
        }
        if mines < 1 {
            return Err("Mines must be greater than 0".to_string());
        }
        if mines >= height * width {
            return Err("Mines must be less than the board size".to_string());
        }

        Ok(Args {
            width,
            height,
            mines,
        })
    }
}
