use std::{error::Error, io, thread::sleep, time::Duration};

use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind},
    style::{Color, Stylize},
    terminal,
};

use crate::render::Object;

mod render;
mod square;

fn main() -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let window_size = terminal::size()?;

    let mut square = square::Square::new((0, 0), (4, 1), (1, 1), ' '.stylize(), Color::Cyan);

    let frame_delay = 50_u64; // in millisecond

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
        }

        let mut fbuffer = render::Framebuffer::new(window_size);

        square.render_to_buffer(&mut fbuffer)?;

        fbuffer.print()?;

        square.update_position(window_size);

        sleep(Duration::from_millis(frame_delay));
    }

    terminal::disable_raw_mode()?;
    Ok(())
}
