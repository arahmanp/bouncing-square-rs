use crossterm::style::Color;

use crate::square;

#[derive(Debug)]
pub struct App {
    window_size: (u16, u16),
    square: square::Square,
    color_list: Vec<Color>,
    active_color: Color,
    frame_delay: u64, // in millis
}
