//#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use anyhow::Result;
use coffee::graphics::WindowSettings;
use coffee::ui::UserInterface;

use crate::game::{SnakeGame, WINDOW_SIZE_X, WINDOW_SIZE_Y};

mod game;
mod snake;

fn main() -> Result<()> {
    <SnakeGame as UserInterface>::run(WindowSettings {
        title: String::from("A baked snake"),
        size: (WINDOW_SIZE_X as u32, WINDOW_SIZE_Y as u32),
        resizable: false,
        fullscreen: false,
        maximized: false,
    })?;
    Ok(())
}
