use crossterm::style::{Color, StyledContent, Stylize};

use crate::render;

#[derive(Debug)]
pub struct Square {
    start: (u16, u16),
    end: (u16, u16),
    direction: (i16, i16),
    texture: StyledContent<char>,
    color: Color,
}

impl Square {
    pub fn new(
        start: (u16, u16),
        end: (u16, u16),
        direction: (i16, i16),
        texture: StyledContent<char>,
        color: Color,
    ) -> Self {
        Square {
            start,
            end,
            direction,
            texture,
            color,
        }
    }

    pub fn update_position(&mut self, window_size: (u16, u16)) {
        let (window_width, window_height) = window_size;
        let (start_col, start_row) = self.start;
        let (end_col, end_row) = self.end;
        let (d_col, d_row) = self.direction;

        if (start_col == 0 && d_col == -1) || (end_col == window_width - 1 && d_col == 1) {
            self.direction.0 *= -1;
        }

        if (start_row == 0 && d_row == -1) || (end_row == window_height - 1 && d_row == 1) {
            self.direction.1 *= -1
        }

        let new_start = (
            (start_col as i16 + self.direction.0) as u16,
            (start_row as i16 + self.direction.1) as u16,
        );

        let new_end = (
            (end_col as i16 + self.direction.0) as u16,
            (end_row as i16 + self.direction.1) as u16,
        );

        self.start = new_start;
        self.end = new_end;
    }

    pub fn change_color(&mut self, new_color: Color) {
        self.color = new_color;
    }
}

impl render::Object for Square {
    fn render_to_buffer(&self, buffer: &mut render::Framebuffer) -> Result<(), String> {
        let (start_col, start_row) = self.start;
        let (end_col, end_row) = self.end;

        for row in start_row..=end_row {
            for col in start_col..=end_col {
                let pixel = self.texture.on(self.color);
                buffer.set_pixel_at(col, row, pixel)?;
            }
        }
        Ok(())
    }
}
