#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![feature(deque_range)]
#![feature(test)]
#![feature(map_first_last)]

use crate::game::{SnakeGame, WINDOW_SIZE_X, WINDOW_SIZE_Y};
use anyhow::Result;
use coffee::graphics::WindowSettings;
use coffee::ui::UserInterface;

mod game;
mod search;
mod snake;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

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
