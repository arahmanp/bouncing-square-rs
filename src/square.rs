#[derive(Debug)]
pub struct Square {
    start: (u16, u16),
    end: (u16, u16),
    direction: (i16, i16),
}

impl Square {
    pub fn new(start: (u16, u16), end: (u16, u16), direction: (i16, i16)) -> Self {
        Square {
            start,
            end,
            direction,
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
}
