use crossterm::style::{StyledContent, Stylize};

#[derive(Debug)]
pub struct Framebuffer {
    buffer: Vec<StyledContent<char>>,
    size: (u16, u16),
}

impl Framebuffer {
    fn new(window_size: (u16, u16)) -> Self {
        let (window_width, window_height) = window_size;

        Framebuffer {
            buffer: vec![' '.stylize(); (window_width * window_height) as usize],
            size: window_size,
        }
    }

    fn get_pixel_at(&self, col: u16, row: u16) -> Result<StyledContent<char>, &str> {
        if col >= self.size.0 || row >= self.size.1 {
            return Err("Pixel position invalid!");
        }

        let col = col as usize;
        let row = row as usize;

        Ok(self.buffer[row * (self.size.0 as usize) + col])
    }

    fn set_pixel_at(&mut self, col: u16, row: u16, c: char) -> Result<(), &str> {
        if col >= self.size.0 || row >= self.size.1 {
            return Err("Pixel position invalid!");
        }

        let col = col as usize;
        let row = row as usize;

        self.buffer[row * (self.size.0 as usize) + col] = c.stylize();

        Ok(())
    }
}
