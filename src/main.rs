// TODO: THe spam directions bug.
// TODO: Scoreboard, items enemies, Pause
//
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

struct Game {
    gl: GlGraphics,
    snake: Snake,
    scale: f64,
    state: State,
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics;

        let BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(BLACK, gl);
        });

        self.snake.render(&mut self.gl, &self.scale, arg)
    }

    fn update(&mut self) {
        if self.state == State::Running {
            self.snake.update(&self.state);
        } else {
            ()
        }
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();
        if self.state == State::Running {
            self.snake.dir = match btn {
                &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
                &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
                &Button::Keyboard(Key::Left) if last_direction != Direction::Right => {
                    Direction::Left
                }
                &Button::Keyboard(Key::Right) if last_direction != Direction::Left => {
                    Direction::Right
                }
                _ => last_direction,
            }
        };
    }
}

struct Snake {
    body: LinkedList<(f64, f64)>,
    dir: Direction,
    scale: f64,
    starting_loc: LinkedList<(f64, f64)>,
}

impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, scale: &f64, args: &RenderArgs) {
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
            })
        })
    }

    fn update(&mut self, state: &State) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        // check if it has reached the end.
        fn check_border(head: &(f64, f64)) -> bool {
            if head.0 >= 10.0 || head.1 >= 10.0 || head.0 < 0.0 || head.1 < 0.0 {
                return true;
            } else {
                return false;
            }
        }

        match check_border(&new_head) {
            true => {
                self.body = LinkedList::from_iter((vec![(1.0, 0.0), (0.0, 0.0)]).into_iter());
                self.dir = Direction::Right;
            }
            false if state == &State::Running => {
                match self.dir {
                    Direction::Left => new_head.0 -= 1.0,
                    Direction::Right => new_head.0 += 1.0,
                    Direction::Up => new_head.1 -= 1.0,
                    Direction::Down => new_head.1 += 1.0,
                }

                self.body.push_front(new_head);

                self.body.pop_back().unwrap();
            }
            _ => (),
        }
    }
}

fn main() {
    const SCALE: f64 = 100.0;
    const DIM: f64 = 100.0 * 10.0;
    const INITIAL_DIR: Direction = Direction::Right;
    let INITIAL: LinkedList<(f64, f64)> =
        LinkedList::from_iter((vec![(1.0, 0.0), (0.0, 0.0)]).into_iter());

    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Snake", [DIM, DIM])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: INITIAL.clone(),
            dir: INITIAL_DIR.clone(),
            scale: SCALE,
            starting_loc: INITIAL.clone(),
        },
        scale: SCALE,
        state: State::Running,
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update()
        }

        if let Some(mc) = e.mouse_cursor_args(){
            println!("{:?}", mc);
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
