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

        if (self.start.0 == 0 && self.direction.0 == -1)
            || (self.end.0 == window_width - 1 && self.direction.0 == 1)
        {
            self.direction.0 *= -1;
        }

        if (self.start.1 == 0 && self.direction.1 == -1)
            || (self.end.1 == window_height - 1 && self.direction.1 == 1)
        {
            self.direction.1 *= -1
        }

        let new_start = (
            (self.start.0 as i16 + self.direction.0) as u16,
            (self.start.1 as i16 + self.direction.1) as u16,
        );

        let new_end = (
            (self.end.0 as i16 + self.direction.0) as u16,
            (self.end.1 as i16 + self.direction.1) as u16,
        );

        self.start = new_start;
        self.end = new_end;
    }
}
