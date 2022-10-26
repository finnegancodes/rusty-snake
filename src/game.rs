use piston_window::{clear, color::hex, rectangle, Context, G2d, GfxDevice, Glyphs, Key};

use crate::{
    apple::Apple,
    direction::Direction,
    hud::HUD,
    snake::{Position, Snake},
    HEIGHT, WIDTH,
};

pub const BG_COLOR: &str = "1a1b4b";
pub const BORDER_COLOR: &str = "292a73";
pub const APPLE_COLOR: &str = "f7a400";

pub const GRID_SIZE: f64 = 25.;
pub const BLOCK_WIDTH: f64 = WIDTH / GRID_SIZE;
pub const MOVE_INTERVAL: f64 = 0.2;

pub struct Game {
    snake: Snake,
    apple: Apple,
    wait_time: f64,
    paused: bool,
    dead: bool,
    score: i8,
    hud: HUD,
}

impl Game {
    pub fn new(glyphs: Glyphs) -> Game {
        let hud = HUD::new(glyphs);

        Game {
            snake: Snake::new(),
            apple: Apple::new(),
            wait_time: 0.,
            paused: false,
            dead: false,
            score: 0,
            hud,
        }
    }

    pub fn draw(&mut self, c: Context, g: &mut G2d, d: &mut GfxDevice) {
        clear(hex(BG_COLOR), g);

        self.apple.draw(c, g);
        self.snake.draw(c, g);
        self.draw_game_borders(c, g);
        self.hud.draw(c, g, d);

        if self.dead {
            self.hud.draw_death_screen(c, g);
        }
    }

    fn draw_game_borders(&self, c: Context, g: &mut G2d) {
        let color = hex(BORDER_COLOR);
        rectangle(color, [0.0, 0.0, WIDTH, BLOCK_WIDTH], c.transform, g);
        rectangle(color, [0.0, 0.0, BLOCK_WIDTH, HEIGHT], c.transform, g);
        rectangle(
            color,
            [0.0, HEIGHT - BLOCK_WIDTH, WIDTH, BLOCK_WIDTH],
            c.transform,
            g,
        );
        rectangle(
            color,
            [WIDTH - BLOCK_WIDTH, 0.0, BLOCK_WIDTH, HEIGHT],
            c.transform,
            g,
        );
    }

    pub fn update(&mut self, dt: f64) {
        if self.paused || self.dead {
            return;
        }
        self.wait_time += dt;

        if self.wait_time >= MOVE_INTERVAL {
            self.snake.update();

            let is_collision = self.snake.check_collision();
            if is_collision {
                self.die()
            };

            self.check_apple();

            self.hud.update(self.score);

            self.wait_time = 0.;
        }
    }

    fn check_apple(&mut self) {
        let head = self.snake.head;
        let apple = self.apple.pos;
        if head.x == apple.x && head.y == apple.y {
            self.snake.tail.grow();
            self.apple.new_position();
            self.score += 1;
        }
    }

    fn change_direction(&mut self, dir: Direction) {
        if !self.paused && !self.dead {
            self.snake.direction = dir;
        }
    }

    fn die(&mut self) {
        self.dead = true;
    }

    fn reset(&mut self) {
        self.snake = Snake::new();
        self.apple = Apple::new();
        self.score = 0;
        self.dead = false;
    }

    pub fn key_pressed(&mut self, key: Key) {
        match key {
            Key::W => {
                self.change_direction(Direction::UP);
            }
            Key::S => {
                self.change_direction(Direction::DOWN);
            }
            Key::A => {
                self.change_direction(Direction::LEFT);
            }
            Key::D => {
                self.change_direction(Direction::RIGHT);
            }
            Key::Space => {
                if self.dead {
                    self.reset();
                    return;
                }
                self.paused = !self.paused;
            }
            Key::P => {
                self.apple.new_position();
            }
            _ => {}
        }
    }
}

pub fn real_position(pos: Position) -> Position {
    let (x, y) = (
        (pos.x * BLOCK_WIDTH) + BLOCK_WIDTH,
        (pos.y * BLOCK_WIDTH) + BLOCK_WIDTH,
    );
    return Position { x, y };
}
