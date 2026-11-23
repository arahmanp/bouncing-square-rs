use std::{error::Error, io, thread::sleep, time::Duration};

use crossterm::{
    ExecutableCommand, cursor, event::{self, Event, KeyCode, KeyEventKind}, style::{Color, Stylize}, terminal,
};

use crate::{render::{Framebuffer, Object}, square::Square};

#[derive(Debug)]
pub struct App {
    window_size: (u16, u16),
    square: Square,
    color_list: Vec<Color>,
    frame_delay: u64, // in millis
}

impl App {
    pub fn init() -> io::Result<Self> {
        let window_size = terminal::size()?;

        let color_list = vec![
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Magenta,
        ];

        let idx = fastrand::usize(..color_list.len());
        let initial_color = color_list[idx];

        let square = Square::new((0, 0), (5, 2), (1, 1), ' '.stylize(), initial_color);

        Ok(App {
            window_size,
            square,
            color_list,
            frame_delay: 500,
        })
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode()?;
        
        stdout
            .execute(terminal::Clear(terminal::ClearType::All))?
            .execute(cursor::Hide)?;

        loop {
            if event::poll(Duration::from_millis(0))? {
                if let Event::Key(k) = event::read()? {
                    if k.kind == KeyEventKind::Press {
                        match k.code {
                            KeyCode::Char('q') => break,
                            _ => {}
                        }
                    }
                }

                if let Event::Resize(new_width, new_height) = event::read()? {
                    self.window_size = (new_width, new_height);
                }
            }

            let mut fbuffer = Framebuffer::new(self.window_size);

            self.square.render_to_buffer(&mut fbuffer)?;

            fbuffer.print()?;

            if self.square.update_position(self.window_size) {
                let idx = fastrand::usize(..self.color_list.len());
                let new_color = self.color_list[idx];
                self.square.change_color(new_color);
            }

            sleep(Duration::from_millis(self.frame_delay));
        }

        Ok(())
    }

    pub fn end(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        stdout
            .execute(terminal::Clear(terminal::ClearType::All))?
            .execute(cursor::MoveTo(0, 0))?;

        terminal::disable_raw_mode()?;

        Ok(())
    }
}
