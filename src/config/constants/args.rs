pub struct Args {
    pub width: u32,
    pub height: u32,
    pub mines: u32,
}

impl Args {
    pub fn new(width: u32, height: u32, mines: u32) -> Result<Args, String> {
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
