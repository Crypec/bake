#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![feature(deque_range)]
#![feature(test)]
use crate::game::{SnakeGame, WINDOW_SIZE_X, WINDOW_SIZE_Y};
use anyhow::Result;
use coffee::graphics::WindowSettings;
use coffee::ui::UserInterface;

mod game;
mod parse;
mod search;
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
