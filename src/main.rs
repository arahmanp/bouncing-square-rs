use std::error::Error;

mod app;
mod render;
mod square;

use app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::init()?;

    app.start()?;

    app.end()?;

    Ok(())
}
