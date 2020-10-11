use crate::game::{
	Direction, NODE_SIZE, WINDOW_SIZE_X, WINDOW_SIZE_X_F32, WINDOW_SIZE_Y, WINDOW_SIZE_Y_F32,
};
use coffee::graphics::{Color, Frame, Mesh, Rectangle, Shape};
use rand::Rng;

const START_SNAKE_LEN: usize = 6;

#[derive(Debug)]
pub struct Snake {
	pub tail: Vec<Position>,
	pub direction: Option<Direction>,
}

impl Snake {
	pub fn new() -> Self {
		let tail = (1..START_SNAKE_LEN)
			.into_iter()
			.map(|i| i as f32)
			.map(|i| Position {
				x: i * NODE_SIZE,
				y: 2.0 * NODE_SIZE,
			})
			.collect::<Vec<_>>();
		Self {
			tail,
			direction: None,
		}
	}

	pub fn ate_itself(&self) -> bool {
		let head = self.tail.last().cloned().unwrap();
		self.is_inside(head)
	}

	pub fn is_inside(&self, pos: Position) -> bool {
		let len = self.tail.len();
		self.tail[..len - 1].iter().any(|node| *node == pos)
	}

	pub fn update(&mut self) {
		if self.tail.len() >= START_SNAKE_LEN {
			self.tail.remove(0);
		}
		let head = self.tail.last().cloned().unwrap();
		match self.direction {
			Some(Direction::Up) => {
				self.tail.push(Position {
					x: head.x,
					y: head.y - NODE_SIZE,
				});
			}
			Some(Direction::Down) => self.tail.push(Position {
				x: head.x,
				y: head.y + NODE_SIZE,
			}),
			Some(Direction::Left) => self.tail.push(Position {
				x: head.x - NODE_SIZE,
				y: head.y,
			}),
			Some(Direction::Right) => self.tail.push(Position {
				x: head.x + NODE_SIZE,
				y: head.y,
			}),
			None => return,
		};
		self.teleport_if_outside()
	}

	fn teleport_if_outside(&mut self) {
		let head = self.tail.last_mut().unwrap();
		match self.direction {
			Some(Direction::Up) if head.y < 0.0 => head.y = WINDOW_SIZE_Y_F32,
			Some(Direction::Down) if head.y > WINDOW_SIZE_Y_F32 => head.y = 0.0,
			Some(Direction::Left) if head.x < 0.0 => head.x = WINDOW_SIZE_X_F32,
			Some(Direction::Right) if head.x > WINDOW_SIZE_X_F32 => head.x = 0.0,
			_ => {}
		}
	}

	const fn direction_is_legal(&self, direction: Direction) -> bool {
		!matches!(
			(self.direction, direction),
			(Some(Direction::Up), Direction::Down)
				| (Some(Direction::Down), Direction::Up)
				| (Some(Direction::Left), Direction::Right)
				| (Some(Direction::Right), Direction::Left)
		)
	}

	pub fn set_direction(&mut self, direction: Direction) {
		if self.direction_is_legal(direction) {
			self.direction = Some(direction)
		}
	}

	pub fn add_node(&mut self, pos: Position) {
		self.tail.push(pos)
	}
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Position {
	pub x: f32,
	pub y: f32,
}

impl Position {
	pub fn dist(self, rhs: Self) -> f32 {
		let dx = rhs.x - self.x;
		let dy = rhs.y - self.y;
		f32::sqrt(dy.mul_add(dy, dx * dx))
	}
}

#[derive(Debug)]
pub struct Apple {
	pub pos: Position,
	pub eaten: bool,
}

impl Apple {
	pub fn new() -> Self {
		Self {
			pos: Self::rand_pos(),
			eaten: false,
		}
	}

	fn rand_pos() -> Position {
		let mut rng = rand::thread_rng();
		let upper_x_bound = WINDOW_SIZE_X / NODE_SIZE as usize;
		let upper_y_bound = WINDOW_SIZE_Y / NODE_SIZE as usize;
		Position {
			x: NODE_SIZE * rng.gen_range(0, upper_x_bound) as f32,
			y: NODE_SIZE * rng.gen_range(0, upper_y_bound) as f32,
		}
	}

	pub fn draw(&self, frame: &mut Frame) {
		let mut mesh = Mesh::new();
		mesh.fill(
			Shape::Rectangle(Rectangle {
				x: self.pos.x,
				y: self.pos.y,
				width: NODE_SIZE,
				height: NODE_SIZE,
			}),
			Color::RED,
		);
		mesh.draw(&mut frame.as_target())
	}
}
