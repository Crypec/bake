use crate::search::*;
use coffee::graphics::{Color, Frame, Mesh, Point, Rectangle, Shape, Window};
use coffee::input::keyboard::KeyCode;
use coffee::input::{self, keyboard, ButtonState, Input};
use coffee::load::Task;
use coffee::ui::{Column, Element, Justify, Renderer, Text, UserInterface};
use coffee::{Game, Timer};

use crate::snake::{Apple, Position, Snake, Tail};

pub const WINDOW_SIZE_X: usize = 600;
pub const WINDOW_SIZE_Y: usize = WINDOW_SIZE_X;

pub const WINDOW_SIZE_X_F32: f32 = WINDOW_SIZE_X as f32;
pub const WINDOW_SIZE_Y_F32: f32 = WINDOW_SIZE_Y as f32;

pub const NODE_SIZE: usize = 30;
pub const NODE_SIZE_F32: f32 = NODE_SIZE as f32;

const GRID_COLOR: Color = Color {
    r: 0.12941,
    g: 0.27843,
    b: 0.32157,
    a: 1.0,
};

const HAM_PATH_COLOR: Color = Color {
    r: 0.98039,
    g: 0.01961,
    b: 0.75686,
    a: 1.0,
};

const PATH_COLOR: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 0.2,
};

const SNAKE_COLOR: Color = Color {
    r: 0.23922,
    g: 0.78039,
    b: 0.06275,
    a: 1.0,
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
    mode: Mode,
    solver: Solver,
    is_finished: bool,
    dump_index: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Mode {
    Computer(DrawMode),
    Human,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum DrawMode {
    Normal,
    Path,
}

pub const GAME_LOWER_BOUND: Position = Position { x: 0, y: 0 };

pub const GAME_UPPER_BOUND: Position = Position {
    x: WINDOW_SIZE_X as isize,
    y: WINDOW_SIZE_Y as isize,
};

impl SnakeGame {
    pub fn new() -> Self {
        let snake = Snake::new();
        let mut solver = Solver::new();
        solver.gen_zig_zag_path();
        solver.init(&snake.tail);
        Self {
            snake,
            speed: 20,
            tick: 0,
            score: 0,
            mode: Mode::Human,
            apple: Apple::new(),
            is_finished: false,
            dump_index: 0,
            solver,
        }
    }

    fn reset(&mut self) {
        self.snake = Snake::new();
        self.spawn_new_apple();
        self.score = 0;
        self.mode = Mode::Human;
        self.solver.init(&self.snake.tail);
    }

    fn toggle_mode(&mut self) {
        match self.mode {
            Mode::Human => self.mode = Mode::Computer(DrawMode::Normal),
            Mode::Computer(_) => self.mode = Mode::Human,
        }
    }

    fn toggle_draw_mode(&mut self) {
        match self.mode {
            Mode::Computer(DrawMode::Normal) => self.mode = Mode::Computer(DrawMode::Path),
            Mode::Computer(DrawMode::Path) => self.mode = Mode::Computer(DrawMode::Normal),
            _ => (),
        }
    }

    fn ate_apple(&self) -> bool {
        let head = self.snake.head().unwrap();
        self.apple.pos == *head
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

    fn draw_tail(&self, mesh: &mut Mesh) {
        for node in &self.snake.tail {
            mesh.fill(
                Shape::Rectangle(Rectangle {
                    x: node.x as f32,
                    y: node.y as f32,
                    width: NODE_SIZE_F32,
                    height: NODE_SIZE_F32,
                }),
                SNAKE_COLOR,
            );
        }
    }

    fn draw_ham_path(&self, mesh: &mut Mesh) {
        let mut points = vec![];
        let half_node = (NODE_SIZE / 2) as isize;
        for pos in &self.solver.path {
            let x = pos.x + half_node;
            let y = pos.y + half_node;
            let point = Point::new(x as f32, y as f32);
            points.push(point);
        }
        let line = Shape::Polyline { points };

        mesh.stroke(line, HAM_PATH_COLOR, 1.0);
    }

    fn draw_grid(mesh: &mut Mesh) {
        let x_bound = WINDOW_SIZE_X / NODE_SIZE as usize;
        let y_bound = WINDOW_SIZE_Y / NODE_SIZE as usize;
        for i in 0..x_bound {
            let i = i as f32;
            let line = Shape::Polyline {
                points: vec![
                    Point::new(i * NODE_SIZE_F32, 0.0),
                    Point::new(i * NODE_SIZE_F32, WINDOW_SIZE_Y_F32),
                ],
            };
            mesh.stroke(line, GRID_COLOR, 1.0);
        }
        for i in 0..y_bound {
            let i = i as f32;
            let line = Shape::Polyline {
                points: vec![
                    Point::new(0.0, i * NODE_SIZE_F32),
                    Point::new(WINDOW_SIZE_X_F32, i * NODE_SIZE_F32),
                ],
            };
            mesh.stroke(line, GRID_COLOR, 1.0);
        }
    }

    fn draw_path(&self, path: &Tail, mesh: &mut Mesh) {
        let mid = NODE_SIZE_F32 / 2.0;

        for pos in path {
            mesh.fill(
                Shape::Rectangle(Rectangle {
                    x: pos.x as f32,
                    y: pos.y as f32,
                    width: NODE_SIZE_F32,
                    height: NODE_SIZE_F32,
                }),
                PATH_COLOR,
            );
        }

        let mut points = path
            .iter()
            .map(|p| (p.x as f32, p.y as f32))
            .map(|(x, y)| Point::new(x + mid, y + mid))
            .collect::<Vec<Point>>();
        let head = self.snake.head().unwrap();

        points.push(Point::new(head.x as f32 + mid, head.y as f32 + mid));
        let line = Shape::Polyline { points };

        mesh.stroke(line, Color::RED, 2.0);
    }

    fn is_outside(&self) -> bool {
        let head = self.snake.head().unwrap();
        !head.in_range(GAME_LOWER_BOUND, GAME_UPPER_BOUND)
    }
}

impl Game for SnakeGame {
    const TICKS_PER_SECOND: u16 = 1000;
    type Input = CustomInput;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Self> {
        Task::succeed(Self::new)
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        let mut mesh = Mesh::new();
        frame.clear(BG_COLOR);
        Self::draw_grid(&mut mesh);
        self.draw_tail(&mut mesh);
        self.apple.draw(&mut mesh);
        if self.mode == Mode::Computer(DrawMode::Path) {
            //self.draw_path(&mut mesh);
            self.draw_ham_path(&mut mesh);
        }
        mesh.draw(&mut frame.as_target())
    }

    fn update(&mut self, _: &Window) {
        if let Mode::Computer(_) = self.mode {
            self.solver
                .make_move(&self.snake.tail, self.apple.pos)
                .expect("failed to make move in solver");
            self.snake.direction = self.solver.make_move(&self.snake.tail, self.apple.pos);
        }

        self.snake.update();
        if self.is_outside() || self.snake.ate_itself() {
            self.reset();
            return;
        }
        if self.ate_apple() {
            self.score += 1;
            self.snake.add_node(self.apple.pos);
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
            Some(KeyCode::Q) => self.toggle_mode(),
            Some(KeyCode::T) => self.toggle_draw_mode(),
            Some(KeyCode::R) => self.reset(),
            Some(KeyCode::Escape) => self.is_finished = true,
            _ => {}
        }
    }

    fn is_finished(&self) -> bool {
        self.is_finished
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
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
        let direction = match self.snake.direction {
            Some(dir) => format!("Direction: {:#?}", dir),
            None => "Standing still".into(),
        };

        Column::new()
            .padding(20)
            .spacing(20)
            .width(window.width() as u32)
            .height(window.height() as u32)
            .justify_content(Justify::End)
            .push(Text::new(&score))
            .push(Text::new(&direction))
            .into()
    }
}
