use std::io;

use crossterm::{
    style::{Color, Stylize},
    terminal,
};

use crate::square::Square;

#[derive(Debug)]
pub struct App {
    window_size: (u16, u16),
    square: Square,
    color_list: Vec<Color>,
    active_color: Color,
    frame_delay: u64, // in millis
}

impl App {
    fn init() -> io::Result<Self> {
        let window_size = terminal::size()?;

        let color_list = vec![
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Magenta,
        ];

        let idx = fastrand::usize(..color_list.len());
        let active_color = color_list[idx];

        let square = Square::new((0, 0), (5, 2), (1, 1), ' '.stylize(), active_color);

        Ok(App {
            window_size,
            square,
            color_list,
            active_color,
            frame_delay: 500,
        })
    }
}
