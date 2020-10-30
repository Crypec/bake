use crate::game::*;
use crate::snake::*;

#[derive(Debug)]
pub struct Maze {
	pub start: Position,
	pub end: Position,
	pub obstacles: Vec<Position>,
}

impl Maze {
	pub fn parse(input: Vec<&str>) -> Self {
		let mut obstacles = Vec::new();
		let mut start = Position::default();
		let mut end = Position::default();
		let mut last = 'a';
		for (y, line) in input.iter().enumerate() {
			for (x, c) in line.chars().enumerate() {
				match c {
					'#' => obstacles.push(Position {
						x: (x * NODE_SIZE) as isize,
						y: (y * NODE_SIZE) as isize,
					}),
					'S' => {
						start = Position {
							x: (x * NODE_SIZE) as isize,
							y: (y * NODE_SIZE) as isize,
						}
					}
					'E' => {
						end = Position {
							x: (x * NODE_SIZE) as isize,
							y: (y * NODE_SIZE) as isize,
						}
					}
					' ' if last == '#' => {
						obstacles.push(Position {
							x: (x * NODE_SIZE) as isize,
							y: (y * NODE_SIZE) as isize,
						});
					}
					' ' => {},
					_ => unreachable!(),
				}
				last = c;
			}
			println!();
		}
		Self {
			start,
			end,
			obstacles,
		}
	}
}
