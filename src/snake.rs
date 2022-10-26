use piston_window::{color::hex, rectangle, Context, G2d};

use crate::{
    direction::Direction,
    game::{BLOCK_WIDTH, GRID_SIZE, real_position}, tail::Tail,
};

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub const SNAKE_COLOR: &str = "3a9efd";

pub struct Snake {
    pub head: Position,
    pub direction: Direction,
    pub tail: Tail,
    vel: Position,
}

impl Snake {
    pub fn new() -> Snake {
        let head = Position { x: 3., y: 3. };
        let direction = Direction::RIGHT;
        let tail = Tail::new();
        let vel = Position { x: 1., y: 0. };

        Snake {
            head,
            tail,
            direction,
            vel,
        }
    }

    pub fn draw(&self, c: Context, g: &mut G2d) {
        self.tail.draw(c, g);
        self.draw_head(c, g);
    }

    fn draw_head(&self, c: Context, g: &mut G2d) {
        let pos = real_position(self.head);
        rectangle(
            hex(SNAKE_COLOR),
            [pos.x, pos.y, BLOCK_WIDTH, BLOCK_WIDTH],
            c.transform,
            g,
        );
    }

    pub fn update(&mut self) {
        self.vel = self.match_dir_to_vel(self.direction);

        self.tail.update(self.head);

        self.head.x += self.vel.x;
        self.head.y += self.vel.y;
    }

    pub fn check_collision(&self) -> bool {
        let head = self.head;
        if (head.x < 0. || head.x > GRID_SIZE - 3.) || (head.y < 0. || head.y > GRID_SIZE - 3.) {
            return true;
        }
        for block in &self.tail.body {
            if block.x == head.x && block.y == head.y {
                return true;
            }
        }
        return false;
    }

    fn match_dir_to_vel(&self, dir: Direction) -> Position {
        match dir {
            Direction::UP => {
                if self.vel.y != 1. {
                    return Position { x: 0., y: -1. };
                }
            },
            Direction::DOWN => {
                if self.vel.y != -1. {
                    return Position { x: 0., y: 1. };
                }
            },
            Direction::LEFT => {
                if self.vel.x != 1. {
                    return Position { x: -1., y: 0. };
                }
            },
            Direction::RIGHT => {
                if self.vel.x != -1. {
                    return Position { x: 1., y: 0. };
                }
            },
        }
        return self.vel;
    }
}

// fn clamp(value: f64, min: f64, max: f64) -> f64 {
//     if value > max {
//         return max;
//     }
//     if value < min {
//         return min;
//     }
//     return value;
// }
