use std::{error::Error, io, thread::sleep, time::Duration};

use crossterm::{
    ExecutableCommand,
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

    let (window_width, window_height) = terminal::size()?;

    let mut buffer = render::Framebuffer::new((window_width, window_height));

    let square = square::Square::new((1, 1), (4, 4), (1, 1), ' '.stylize(), Color::Cyan);

    square.render_to_buffer(&mut buffer)?;

    buffer.print()?;

    sleep(Duration::from_secs(5));

    terminal::disable_raw_mode()?;
    Ok(())
}
