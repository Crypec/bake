use crate::game::*;
use crate::snake::*;
use rustc_hash::FxHashSet;
use smallvec::SmallVec;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const ROTATION_MATRIX: [(isize, isize); 4] = [
	(0, -1),
	(0, 1),
	(-1, 0),
	(1, 0),
	// (-1, -1),
	// (1, -1),
	// (-1, 1),
	// (1, 1),
];

const MAX_NODE_LINK_LEN: usize = (WINDOW_SIZE_X / NODE_SIZE) * (WINDOW_SIZE_Y / NODE_SIZE);

extern crate test;

#[derive(Debug, Clone, Copy)]
pub struct Node {
	pos: Position,
	parent_id: Option<usize>,
	id: usize,
	g_cost: f32,
	h_cost: f32,
	f_cost: f32,
}

impl Ord for Node {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.f_cost > other.f_cost {
			Ordering::Less
		} else if self.f_cost < other.f_cost {
			Ordering::Greater
		} else {
			Ordering::Equal
		}
	}
}

impl Default for Node {
	fn default() -> Self {
		Self {
			pos: Position::default(),
			parent_id: None,
			id: 0,
			g_cost: 0.0,
			h_cost: 0.0,
			f_cost: 0.0,
		}
	}
}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Node {
	fn eq(&self, other: &Self) -> bool {
		self.f_cost == other.f_cost
	}
}

impl Eq for Node {}

#[derive(Debug)]
pub struct Searcher {
	node_link: Vec<Node>,
	cursor: usize,
	open: BinaryHeap<Node>,
	closed: FxHashSet<Position>,
}

impl Searcher {
	pub fn new() -> Self {
		Self {
			node_link: Vec::with_capacity(MAX_NODE_LINK_LEN),
			cursor: 0,
			open: BinaryHeap::with_capacity(256),
			closed: FxHashSet::default(),
		}
	}

	pub fn clear(&mut self) {
		self.open.clear();
		self.closed.clear();
		self.node_link.clear();
		self.cursor = 0;
	}

	pub fn a_star(
		&mut self,
		start: Position,
		goal: Position,
		obstacles: &[Position],
	) -> Option<Vec<Position>> {
		self.clear();

		let start_node = Node {
			pos: start,
			parent_id: None,
			id: self.gen_id(),
			g_cost: 0.0,
			h_cost: 0.0,
			f_cost: 0.0,
		};

		self.open.push(start_node);
		self.node_link.push(start_node);

		while let Some(current) = self.open.pop() {

			if current.pos == goal {
				return Some(self.backtrack_path(current));
			}

			self.closed.insert(current.pos);

			for mut child in self.gen_childs(&current, obstacles) {
				if self.closed.contains(&child.pos) {
					continue;
				}

				child.g_cost = current.g_cost + 1.0;
				child.h_cost = child.pos.dist(goal);
				child.f_cost = child.g_cost + child.h_cost;

				for open_node in &self.open {
					if child == *open_node && child.g_cost > open_node.g_cost {
						continue;
					}
				}
				self.open.push(child);
			}
		}
		None
	}

	fn backtrack_path(&self, current: Node) -> Vec<Position> {
		let mut path = Vec::with_capacity(256);
		path.push(current.pos);
		let mut current = current;
		while let Some(id) = current.parent_id {
			path.push(current.pos);
			current = *self.node_link.get(id).unwrap();
		}
		path
	}

	#[inline(always)]
	fn gen_childs(&mut self, current: &Node, obstacles: &[Position]) -> SmallVec<[Node; ROTATION_MATRIX.len()]> {
		let mut childs: SmallVec<[Node; ROTATION_MATRIX.len()]> = SmallVec::new();

		for (x, y) in &ROTATION_MATRIX {
			let x_offset = x * NODE_SIZE as isize;
			let y_offset = y * NODE_SIZE as isize;
			let pos = Position {
				x: current.pos.x + x_offset,
				y: current.pos.y + y_offset,
			};

			if obstacles.contains(&pos) || !pos.in_range(GAME_LOWER_BOUND, GAME_UPPER_BOUND) {
				continue;
			}

			let child_node = Node {
				pos,
				parent_id: Some(current.id),
				id: self.gen_id(),
				g_cost: 0.0,
				h_cost: 0.0,
				f_cost: 0.0,
			};
			self.node_link.push(child_node);
			childs.push(child_node);
		}
		childs
	}

	fn gen_id(&mut self) -> usize {
		let id = self.cursor;
		self.cursor += 1;
		id
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::parse::*;
	use test::Bencher;

	#[bench]
	fn benchmark_astar(b: &mut Bencher) {
		let file = std::fs::read_to_string("./maze.txt").unwrap();
		let lines = file.lines().collect();
		let maze = Maze::parse(lines);
		let mut searcher = Searcher::new();
		b.iter(|| searcher.a_star(maze.start, maze.end, &maze.obstacles).unwrap())
	}
}
