use piston_window::{color::hex, rectangle, Context, G2d};
use rand::Rng;

use crate::{
    game::{real_position, APPLE_COLOR, BLOCK_WIDTH, GRID_SIZE},
    snake::Position,
};

pub struct Apple {
    pub pos: Position,
}

impl Apple {
    pub fn new() -> Apple {
        let r = random_position();
        Apple { pos: r }
    }

    pub fn draw(&self, c: Context, g: &mut G2d) {
        let real = real_position(self.pos);
        rectangle(
            hex(APPLE_COLOR),
            [real.x, real.y, BLOCK_WIDTH, BLOCK_WIDTH],
            c.transform,
            g,
        );
    }

    pub fn new_position(&mut self) {
        self.pos = random_position();
    }
}

fn random_position() -> Position {
    let mut rand = rand::thread_rng();
    let max = (GRID_SIZE - 2.) as u8;
    let x = rand.gen_range(0..max) as f64;
    let y = rand.gen_range(0..max) as f64;
    return Position { x, y };
}
