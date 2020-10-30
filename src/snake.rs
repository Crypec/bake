use crate::game::{Direction, NODE_SIZE, NODE_SIZE_F32, WINDOW_SIZE_X, WINDOW_SIZE_Y};
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
			.map(|i| Position {
				x: (i * NODE_SIZE) as isize,
				y: 2 * NODE_SIZE as isize,
			})
			.collect::<Vec<_>>();
		Self {
			tail,
			direction: None,
		}
	}

	pub fn ate_itself(&self) -> bool {
		let head = self.head().unwrap();
		self.is_inside(*head)
	}

	pub fn is_inside(&self, pos: Position) -> bool {
		let len = self.tail.len();
		self.tail[..len - 1].iter().any(|node| *node == pos)
	}

	pub fn update(&mut self) {
		if self.tail.len() >= START_SNAKE_LEN {
			self.tail.remove(0);
		}

		let head = self.head().cloned().unwrap();
		match self.direction {
			Some(Direction::Up) => {
				self.tail.push(Position {
					x: head.x,
					y: head.y - NODE_SIZE as isize,
				});
			}
			Some(Direction::Down) => self.tail.push(Position {
				x: head.x,
				y: head.y + NODE_SIZE as isize,
			}),
			Some(Direction::Left) => self.tail.push(Position {
				x: head.x - NODE_SIZE as isize,
				y: head.y,
			}),
			Some(Direction::Right) => self.tail.push(Position {
				x: head.x + NODE_SIZE as isize,
				y: head.y,
			}),
			None => {}
		};
		//self.teleport_if_outside()
	}

	#[allow(dead_code)]
	fn teleport_if_outside(&mut self) {
		let head = self.tail.last_mut().unwrap();
		match self.direction {
			Some(Direction::Down) if head.y > WINDOW_SIZE_Y as isize => head.y = 0,
			Some(Direction::Right) if head.x > WINDOW_SIZE_X as isize => head.x = 0,
			Some(Direction::Up) if head.y < 0 => head.y = WINDOW_SIZE_Y as isize,
			Some(Direction::Left) if head.x < 0 => head.x = WINDOW_SIZE_X as isize,
			_ => {}
		}
	}

	pub const fn direction_is_legal(&self, direction: Direction) -> bool {
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

	pub fn head(&self) -> Option<&Position> {
		self.tail.last()
	}

	#[allow(dead_code)]
	pub fn head_mut(&mut self) -> Option<&mut Position> {
		self.tail.last_mut()
	}
}

#[derive(Debug, PartialEq, Copy, Clone, Hash)]
pub struct Position {
	pub x: isize,
	pub y: isize,
}

impl Position {
	#[allow(dead_code)]
	pub const fn mhtn_dist(self, rhs: Self) -> isize {
		let dx = rhs.x - self.x;
		let dy = rhs.y - self.y;
		isize::abs(dx) + isize::abs(dy)
	}

	pub fn dist(self, rhs: Self) -> f32 {
		let dx = (self.x - rhs.x) as f32;
		let dy = (self.y - rhs.y) as f32;
		dx * dx + dy * dy
	}

	#[inline(always)]
	pub const fn in_range(self, lower: Self, upper: Self) -> bool {
		self.x >= lower.x && self.x < upper.x && self.y >= lower.y && self.y < upper.y
	}
}

impl Eq for Position {}
impl Default for Position {
	fn default() -> Self {
		Self { x: 0, y: 0 }
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
		let upper_x_bound = WINDOW_SIZE_X / NODE_SIZE;
		let upper_y_bound = WINDOW_SIZE_Y / NODE_SIZE;
		Position {
			x: (NODE_SIZE * rng.gen_range(0, upper_x_bound)) as isize,
			y: (NODE_SIZE * rng.gen_range(0, upper_y_bound)) as isize,
		}
	}

	pub fn draw(&self, frame: &mut Frame) {
		let mut mesh = Mesh::new();
		mesh.fill(
			Shape::Rectangle(Rectangle {
				x: self.pos.x as f32,
				y: self.pos.y as f32,
				width: NODE_SIZE_F32,
				height: NODE_SIZE_F32,
			}),
			Color::RED,
		);
		mesh.draw(&mut frame.as_target())
	}
}
