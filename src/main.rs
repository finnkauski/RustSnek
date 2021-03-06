// TODO: THe spam directions bug.
// TODO: Scoreboard, items enemies, Pause
// TODO: Automatically workout the scale dimensions
// TODO: Use resize event to make it scalable.
// TODO: Food spawns outside of box
// NOTE: Multiscreen resolution broken.
// TODO: window resizing
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use std::collections::LinkedList;
use std::iter::FromIterator;

use rand::Rng;

enum Outcome {
    Side,
    Food,
    None,
}

#[derive(Clone, PartialEq)]
enum State {
    Running,
    Paused,
}

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Clone, PartialEq)]
struct Coord {
    x: f64,
    y: f64,
}

struct Edible {
    lifetime: i32,
    color: [f32; 4],
    position: Coord,
    scale: f64,
    max_coords: Coord,
}

impl Edible {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let square: graphics::types::Rectangle = graphics::rectangle::square(
            self.position.x * self.scale,
            self.position.y * self.scale,
            self.scale,
        );

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            graphics::rectangle(self.color, square, transform, gl);
        })
    }

    fn update(&mut self, state: &State) {
        match state {
            State::Running => {
                if self.lifetime == 0 {
                    self.lifetime = 50;
                    self.gen_new_coords()
                } else {
                    self.lifetime -= 1;
                }
            }
            State::Paused => (),
        }
    }

    fn gen_new_coords(&mut self) {
        // rng
        let mut rng = rand::thread_rng();

        let ediX = rng.gen_range(0.0, &self.max_coords.x).round();
        let ediY = rng.gen_range(0.0, &self.max_coords.y).round();
        self.position = Coord { x: ediX, y: ediY };
    }
}

struct Game {
    gl: GlGraphics,
    snake: Snake,
    dot: Edible,
    state: State,
    grid_dims: (f64, f64),
    score: usize,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics;

        let col: f32 = (self.score as f32) / 25.0;
        let black: [f32; 4] = [0.0, 0.0, col, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(black, gl);
        });

        self.snake.render(&mut self.gl, arg)
    }

    fn update(&mut self) {
        if self.state == State::Running {
            match self.check_outcome() {
                Outcome::Side => {
                    self.snake.update(&self.state, Outcome::Side);
                    self.score = 0;
                    self.dot.lifetime = 0;
                }
                Outcome::Food => {
                    self.dot.lifetime = 0;
                    self.score += 1;
                    self.snake.update(&self.state, Outcome::Food);
                }
                Outcome::None => {
                    self.snake.update(&self.state, Outcome::None);
                }
            }
        }
    }

    fn check_outcome(&self) -> Outcome {
        let new_head = self.snake.body.front().expect("snake has no body");
        // check if it has reached the end.
        let hit: Outcome = if new_head.0 >= self.grid_dims.0
            || new_head.1 >= self.grid_dims.1
            || new_head.0 < 0.0
            || new_head.1 < 0.0
        {
            Outcome::Side
        } else if self.dot.position
            == (Coord {
                x: new_head.0,
                y: new_head.1,
            })
        {
            Outcome::Food
        } else {
            Outcome::None
        };
        hit
    }
    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();
        if self.state == State::Running {
            self.snake.dir = match btn {
                &Button::Keyboard(Key::W) if last_direction != Direction::Down => Direction::Up,
                &Button::Keyboard(Key::S) if last_direction != Direction::Up => Direction::Down,
                &Button::Keyboard(Key::A) if last_direction != Direction::Right => Direction::Left,
                &Button::Keyboard(Key::D) if last_direction != Direction::Left => Direction::Right,
                _ => last_direction,
            }
        };
    }
}

struct Snake {
    body: LinkedList<(f64, f64)>,
    dir: Direction,
    scale: f64,
}

impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let white: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self
            .body
            .iter()
            .map(|&(x, y)| graphics::rectangle::square(x * self.scale, y * self.scale, self.scale))
            .collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter().for_each(|square| {
                graphics::rectangle(white, square, transform, gl);
            });
        });
    }

    fn update(&mut self, state: &State, outcome: Outcome) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        fn increment(dir: &Direction, head: &mut (f64, f64)) {
                match dir {
                    Direction::Left => head.0 -= 1.0,
                    Direction::Right => head.0 += 1.0,
                    Direction::Up => head.1 -= 1.0,
                    Direction::Down => head.1 += 1.0,
                }
        }

        match outcome {
            Outcome::Side => {
                self.body = LinkedList::from_iter((vec![(0.0, 0.0)]).into_iter());
                self.dir = Direction::Right;
            }
            Outcome::None if state == &State::Running => {
                increment(&self.dir, &mut new_head);
                
                self.body.push_front(new_head);
                self.body.pop_back().unwrap();
            }
            Outcome::Food if state == &State::Running => {
                increment(&self.dir, &mut new_head);

                self.body.push_front(new_head);
            }
            _ => (),
        }
    }
}

fn main() {
    // settings
    const WINDOW_WIDTH: f64 = 500.0;
    const WINDOW_HEIGHT: f64 = 500.0;
    const SNAKE_DIM: f64 = 25.0;
    const GRID_X: f64 = WINDOW_WIDTH / SNAKE_DIM;
    const GRID_Y: f64 = WINDOW_HEIGHT / SNAKE_DIM;
    const INITIAL_DIR: Direction = Direction::Right;

    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Snake", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // create initial reward
    // TODO: shove into Game
    let dot = Edible {
        lifetime: 0,
        color: [1.0, 0.0, 0.0, 1.0],
        position: Coord { x: -1.0, y: -1.0 },
        scale: SNAKE_DIM,
        max_coords: Coord {
            x: GRID_X,
            y: GRID_Y,
        },
    };

    // create game instance
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0.0, 0.0)]).into_iter()),
            dir: INITIAL_DIR.clone(),
            scale: SNAKE_DIM,
        },
        dot: dot,
        state: State::Running,
        grid_dims: (GRID_X, GRID_Y),
        score: 0,
    };

    // eventloop
    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
            game.dot.render(&mut game.gl, &r);
        }

        if let Some(_u) = e.update_args() {
            game.update();
            game.dot.update(&game.state);
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                match &k.button {
                    &Button::Keyboard(Key::Space) if game.state == State::Running => {
                        game.state = State::Paused
                    }
                    &Button::Keyboard(Key::Space) if game.state == State::Paused => {
                        game.state = State::Running
                    }
                    _ => game.pressed(&k.button),
                }
            }
        }
    }
}
