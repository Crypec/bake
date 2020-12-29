use crate::game::*;
use crate::game::{WINDOW_SIZE_X, WINDOW_SIZE_Y};
use crate::snake::*;
use hashbrown::HashSet;
use std::cmp::Ordering;
use std::collections::{BTreeSet, VecDeque};
use std::hash::{Hash, Hasher};

const NODE_LEN_X: usize = WINDOW_SIZE_X / NODE_SIZE;
const NODE_LEN_Y: usize = WINDOW_SIZE_Y / NODE_SIZE;

type Board = Vec<Vec<Position>>;

const MAX_NODE_LINK_LEN: usize = (WINDOW_SIZE_X / NODE_SIZE) * (WINDOW_SIZE_Y / NODE_SIZE);

#[derive(Debug)]
pub struct Solver {
    pub board: Board,
    pub searcher: Searcher,
    cursor: usize,
    pub path: Vec<Position>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            // TODO(Simon): I might have messed up the order of (x, y)
            board: vec![vec![Position::default(); NODE_LEN_X]; NODE_LEN_Y],
            path: Vec::with_capacity(MAX_NODE_LINK_LEN),
            searcher: Searcher::new(),
            cursor: 0,
        }
    }

    fn new_node(&mut self, x: usize, y: usize) {
        let pos = Position {
            x: (x * NODE_SIZE) as isize,
            y: (y * NODE_SIZE) as isize,
        };
        self.board[x][y] = pos;
        self.path.push(pos)
    }

    fn inc_cursor(&mut self) {
        let end = self.path.len() - 1;
        if self.cursor == end {
            self.cursor = 0;
        } else {
            self.cursor += 1;
        }
    }

    fn next_ham(&mut self) -> Option<Position> {
        let cursor = self.inc_cursor();
        self.path.get(self.cursor - 1).copied()
    }

    pub fn make_move(&mut self, tail: &Tail, apple: Position) -> Option<Direction> {
        let head = tail.front().unwrap();
        let ham_head = self.next_ham()?;

        // let search_head = self
        //	.searcher
        //	.a_star(*head, apple, tail)
        //	.expect("failed to find path")
        //	.last()
        //	.copied()?;

        let ham_index = self
            .get_ham_path_index(ham_head)
            .expect("failed to find ham_index");
        let head_index = self
            .get_ham_path_index(*head)
            .expect("failed to find head_index");
        println!(
            "head: {} :: ham: {} :: cursor: {}",
            head_index, ham_index, self.cursor
        );
        return Position::to_direction(*head, ham_head);
        // dbg!((search_head, ham_head));
        // if search_index > head_index {
        //	println!("use search head");
        // } else {
        //	println!("use ham head");
        //	Position::to_direction(*head, ham_head)
        // }
    }

    pub fn match_starting_pos(&self, tail: &Tail) -> Option<usize> {
        let head = tail.front()?;
        self.get_ham_path_index(*head)
    }

    pub fn init(&mut self, tail: &Tail) {
        let starting_pos = self
            .match_starting_pos(tail)
            .expect("failed to match starting pos to head of snake");
        self.cursor = starting_pos - 1;
    }

    pub fn gen_zig_zag_path(&mut self) {
        for x in 0..NODE_LEN_X {
            match x % 2 {
                0 => {
                    for y in (0..NODE_LEN_Y - 1).rev() {
                        self.new_node(x, y);
                    }
                }
                _ => {
                    for y in 0..NODE_LEN_Y - 1 {
                        self.new_node(x, y);
                    }
                }
            }
        }
        for x in (0..NODE_LEN_X).rev() {
            self.new_node(x, NODE_LEN_Y - 1);
        }
    }

    pub fn get_ham_path_index(&self, pos: Position) -> Option<usize> {
        self.path.iter().position(|p| *p == pos)
    }
}

const ROTATION_MATRIX: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pos: Position,
    parent_id: Option<usize>,
    id: usize,
    g_cost: isize,
    h_cost: isize,
    f_cost: isize,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.f_cost > other.f_cost {
            Ordering::Greater
        } else if self.f_cost < other.f_cost {
            Ordering::Less
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
            g_cost: 0,
            h_cost: 0,
            f_cost: 0,
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
        self.pos == other.pos
    }
}

impl Eq for Node {}

#[derive(Debug)]
pub struct Searcher {
    node_link: Vec<Node>,
    cursor: usize,
    open: BTreeSet<Node>,
    closed: HashSet<Position>,
    obstacles: HashSet<Position>,
    childs: [Node; ROTATION_MATRIX.len()],
}

impl Searcher {
    pub fn new() -> Self {
        Self {
            node_link: Vec::with_capacity(MAX_NODE_LINK_LEN),
            cursor: 0,
            open: BTreeSet::new(),
            obstacles: HashSet::with_capacity(MAX_NODE_LINK_LEN),
            closed: HashSet::with_capacity(MAX_NODE_LINK_LEN),
            childs: [Node::default(); ROTATION_MATRIX.len()],
        }
    }

    pub fn update_obs_cache(&mut self, obstacles: &VecDeque<Position>) {
        self.obstacles.clear();
        for obstacle in obstacles {
            self.obstacles.insert(*obstacle);
        }
    }

    pub fn reset(&mut self) {
        self.open.clear();
        self.closed.clear();
        self.node_link.clear();
        self.obstacles.clear();
        self.cursor = 0;
    }

    pub fn a_star(
        &mut self,
        start: Position,
        goal: Position,
        obstacles: &VecDeque<Position>,
    ) -> Option<Vec<Position>> {
        self.reset();
        self.update_obs_cache(obstacles);

        let start_node = Node {
            pos: start,
            parent_id: None,
            id: self.gen_id(),
            g_cost: 0,
            h_cost: 0,
            f_cost: 0,
        };
        self.open.insert(start_node);
        self.node_link.push(start_node);

        while let Some(current) = self.open.pop_first() {
            debug_assert!(self.open.len() < MAX_NODE_LINK_LEN, "Infinite Loop!");

            self.closed.insert(current.pos);

            if current.pos == goal {
                return Some(self.backtrack_path(current));
            }

            self.new_gen_childs(&current);
            for child in &mut self.childs {
                if self.obstacles.contains(&child.pos)
                    || !child.pos.in_range(GAME_LOWER_BOUND, GAME_UPPER_BOUND)
                    || self.closed.contains(&child.pos)
                {
                    continue;
                }

                child.g_cost = current.g_cost + 1;
                child.h_cost = child.pos.mhtn_dist(goal);
                child.f_cost = child.g_cost + child.h_cost;

                match self.open.get(child) {
                    Some(node) if child.g_cost > node.g_cost => continue,
                    _ => {
                        self.open.insert(*child);
                    }
                }
            }
        }
        None
    }

    fn backtrack_path(&self, current: Node) -> Vec<Position> {
        let mut path = Vec::with_capacity(512);
        path.push(current.pos);
        let mut current = current;
        while let Some(id) = current.parent_id {
            path.push(current.pos);
            unsafe {
                current = *self.node_link.get_unchecked(id);
            }
        }
        path
    }

    fn new_gen_childs(&mut self, current: &Node) {
        for i in 0..ROTATION_MATRIX.len() {
            let (x, y) = ROTATION_MATRIX[i];
            let x_offset = x * NODE_SIZE as isize;
            let y_offset = y * NODE_SIZE as isize;
            let pos = Position {
                x: current.pos.x + x_offset,
                y: current.pos.y + y_offset,
            };

            let child_node = Node {
                pos,
                parent_id: Some(current.id),
                id: self.gen_id(),
                g_cost: 0,
                h_cost: 0,
                f_cost: 0,
            };

            self.childs[i] = child_node;
            self.node_link.push(child_node);
        }
    }

    fn gen_id(&mut self) -> usize {
        let id = self.cursor;
        self.cursor += 1;
        id
    }
}
