use std::{
    error::Error,
    io::{self, Write},
};

use crossterm::{
    QueueableCommand, cursor,
    style::{self, StyledContent, Stylize},
};

#[derive(Debug)]
pub struct Framebuffer {
    buffer: Vec<StyledContent<char>>,
    size: (u16, u16),
}

impl Framebuffer {
    pub fn new(window_size: (u16, u16)) -> Self {
        let (width, height) = window_size;

        Framebuffer {
            buffer: vec![' '.stylize(); (width * height) as usize],
            size: window_size,
        }
    }

    pub fn get_pixel_at(&self, col: u16, row: u16) -> Result<StyledContent<char>, String> {
        let (width, height) = self.size;

        if col >= width || row >= height {
            return Err(String::from("Pixel position invalid!"));
        }

        let col = col as usize;
        let row = row as usize;

        Ok(self.buffer[row * (width as usize) + col])
    }

    pub fn set_pixel_at(
        &mut self,
        col: u16,
        row: u16,
        pixel: StyledContent<char>,
    ) -> Result<(), String> {
        let (width, height) = self.size;

        if col >= width || row >= height {
            return Err(String::from("Pixel position invalid!"));
        }

        let col = col as usize;
        let row = row as usize;

        self.buffer[row * (width as usize) + col] = pixel;

        Ok(())
    }

    pub fn print(&self) -> Result<(), Box<dyn Error>> {
        let mut stdout = io::stdout();

        let (width, height) = self.size;

        for row in 0..height {
            for col in 0..width {
                let pixel = self.get_pixel_at(col, row)?;

                stdout
                    .queue(cursor::MoveTo(col, row))?
                    .queue(style::PrintStyledContent(pixel))?;
            }
        }

        stdout.flush()?;

        Ok(())
    }
}

pub trait Object {
    fn render_to_buffer(&self, buffer: &mut Framebuffer) -> Result<(), String>;
}
