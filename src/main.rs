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
        self.snake.update();
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right)
                if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        }

    }
}

struct Snake {
    pos_x: f64,
    pos_y: f64,
    dir: Direction,
    scale: f64,
}

impl Snake {
    fn render(&mut self, gl: &mut GlGraphics, scale: &f64, args: &RenderArgs) {

        let white: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let square = graphics::rectangle::square(
            (self.pos_x * self.scale) as f64,
            (self.pos_y * self.scale) as f64,
            (self.scale as f64),
        );

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(white, square, transform, gl);
        })
    }

    fn update(&mut self) {
        match self.dir {
            Direction::Left => self.pos_x -= 1.0,
            Direction::Right => self.pos_x += 1.0,
            Direction::Up => self.pos_y -= 1.0,
            Direction::Down => self.pos_y += 1.0,
        }
    }
}

fn main() {
    const SCALE: f64 = 50.0;
    const DIM: f64 = SCALE * 10.0;
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("Snake", [DIM, DIM])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            pos_x: 0.0,
            pos_y: 0.0,
            dir: Direction::Right,
            scale: SCALE,
        },
        scale: SCALE,
    };
    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update()
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
