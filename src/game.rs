use coffee::graphics::{Color, Frame, Mesh, Point, Rectangle, Shape, Window, Image};
use coffee::input::keyboard::KeyCode;
use coffee::input::{self, keyboard, ButtonState, Input};
use coffee::load::Task;
use coffee::ui::{Column, Element, Justify, Renderer, Text, UserInterface};
use coffee::{Game, Timer};
use paris::*;

use crate::snake::{Apple, Snake};

pub const WINDOW_SIZE_X: usize = 600;
pub const WINDOW_SIZE_Y: usize = WINDOW_SIZE_X;

pub const WINDOW_SIZE_X_F32: f32 = WINDOW_SIZE_X as f32;
pub const WINDOW_SIZE_Y_F32: f32 = WINDOW_SIZE_Y as f32;

pub const NODE_SIZE: f32 = 30.0;
pub const PROXIMITY_TRESHOLD: f32 = 2.0;

const MESH_COLOR: Color = Color {
	r: 0.12941,
	g: 0.27843,
	b: 0.32157,
	a: 1.0,
};

const SNAKE_COLOR: Color = Color {
	r: 0.23922,
	g: 0.78039,
	b: 0.06275,
	a: 0.7,
};

const BG_COLOR: Color = Color {
	r: 0.10196,
	g: 0.23529,
	b: 0.28235,
	a: 1.0,
};

#[derive(Debug)]
pub struct SnakeGame {
	snake: Snake,
	apple: Apple,
	speed: u32,
	tick: u32,
	score: u32,
	misc: Misc,
}

#[derive(Debug)]
struct Misc {
	apple: Option<Image>,
}

impl SnakeGame {
	pub fn new() -> Self {
		Self {
			snake: Snake::new(),
			speed: 20,
			tick: 0,
			score: 0,
			apple: Apple::new(),
			misc: Misc {
				apple: None,
			}
		}
	}

	fn reset(&mut self) {
		self.snake = Snake::new();
		self.spawn_new_apple();
		self.score = 0;
	}

	fn ate_apple(&mut self) -> bool {
		let head = self.snake.tail.last().unwrap();
		self.apple.pos.dist(*head) <= PROXIMITY_TRESHOLD
	}

	fn spawn_new_apple(&mut self) {
		loop {
			let apple = Apple::new();
			if !self.snake.is_inside(apple.pos) {
				self.apple = apple;
				return;
			}
		}
	}

	fn draw_tail(&self, frame: &mut Frame) {
		let mut mesh = Mesh::new();
		for node in &self.snake.tail {
			mesh.fill(
				Shape::Rectangle(Rectangle {
					x: node.x,
					y: node.y,
					width: NODE_SIZE,
					height: NODE_SIZE,
				}),
				SNAKE_COLOR,
			);
			mesh.draw(&mut frame.as_target())
		}
	}

	pub fn draw_grid(frame: &mut Frame) {
		let x_bound = WINDOW_SIZE_X / NODE_SIZE as usize;
		let y_bound = WINDOW_SIZE_Y / NODE_SIZE as usize;
		let mut mesh = Mesh::new();
		for i in 0..x_bound {
			let i = i as f32;
			let line = Shape::Polyline {
				points: vec![
					Point::new(i * NODE_SIZE, 0.0),
					Point::new(i * NODE_SIZE, WINDOW_SIZE_Y_F32),
				],
			};
			mesh.stroke(line, MESH_COLOR, 1.0);
		}
		for i in 0..y_bound {
			let i = i as f32;
			let line = Shape::Polyline {
				points: vec![
					Point::new(0.0, i * NODE_SIZE),
					Point::new(WINDOW_SIZE_X_F32, i * NODE_SIZE),
				],
			};
			mesh.stroke(line, MESH_COLOR, 1.0);
		}
		mesh.draw(&mut frame.as_target())
	}
}

impl Game for SnakeGame {
	const TICKS_PER_SECOND: u16 = 12;
	type Input = CustomInput;
	type LoadingScreen = ();

	fn load(_window: &Window) -> Task<Self> {
		Image::load("./misc/apple.png").map(|image| {
			let mut game = Self::new();
			game.misc.apple = Some(image);
			game
		})
	}

	fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
		self.misc.apple.unwrap().draw(frame.as_target());
		frame.clear(BG_COLOR);
		Self::draw_grid(frame);
		self.draw_tail(frame);
		self.apple.draw(frame);
	}

	fn update(&mut self, _window: &Window) {
		self.snake.update();
		if self.snake.ate_itself() {
			self.reset();
		}
		if self.ate_apple() {
			self.score += 1;
			self.snake.add_node(self.apple.pos);
			info!("ate apple");
			self.spawn_new_apple();
		}
	}

	fn interact(&mut self, input: &mut Self::Input, _window: &mut Window) {
		match input.key_code {
			Some(KeyCode::W) | Some(KeyCode::Up) | Some(KeyCode::K) => {
				self.snake.set_direction(Direction::Up);
			}
			Some(KeyCode::A) | Some(KeyCode::Left) | Some(KeyCode::H) => {
				self.snake.set_direction(Direction::Left);
			}
			Some(KeyCode::S) | Some(KeyCode::Down) | Some(KeyCode::J) => {
				self.snake.set_direction(Direction::Down);
			}
			Some(KeyCode::D) | Some(KeyCode::Right) | Some(KeyCode::L) => {
				self.snake.set_direction(Direction::Right);
			}
			Some(KeyCode::R) => self.reset(),
			_ => {}
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Debug)]
pub struct CustomInput {
	key_code: Option<KeyCode>,
}

impl Input for CustomInput {
	fn new() -> Self {
		Self { key_code: None }
	}

	fn update(&mut self, event: input::Event) {
		if let input::Event::Keyboard(keyboard::Event::Input {
			key_code,
			state: ButtonState::Pressed,
		}) = event
		{
			self.key_code = Some(key_code)
		}
	}

	fn clear(&mut self) {
		self.key_code = None
	}
}

impl UserInterface for SnakeGame {
	type Message = ();
	type Renderer = Renderer;

	fn react(&mut self, _: Self::Message, _: &mut Window) {}

	fn layout(&mut self, window: &Window) -> Element<Self::Message> {
		let score = format!("Score: {}", self.score);
		Column::new()
			.padding(20)
			.spacing(20)
			.width(window.width() as u32)
			.height(window.height() as u32)
			.justify_content(Justify::Start)
			.push(Text::new(&score))
			.into()
	}
}
