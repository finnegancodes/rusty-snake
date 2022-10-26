use piston_window::{color::hex, rectangle, Context, G2d};

use crate::{
    game::{real_position, BLOCK_WIDTH},
    snake::{Position, SNAKE_COLOR},
};

pub struct Tail {
    pub body: Vec<Position>,
}

impl Tail {
    pub fn new() -> Tail {
        let body = new_tail_body();
        Tail { body }
    }

    pub fn draw(&self, c: Context, g: &mut G2d) {
        for block in &self.body {
            self.draw_block(*block, c, g);
        }
    }

    fn draw_block(&self, block: Position, c: Context, g: &mut G2d) {
        let real = real_position(block);
        rectangle(
            hex(SNAKE_COLOR),
            [real.x, real.y, BLOCK_WIDTH, BLOCK_WIDTH],
            c.transform,
            g,
        );
    }

    pub fn update(&mut self, head: Position) {
        for i in (1..self.body.len()).rev() {
            self.body[i] = self.body[i - 1];
        }
        self.body[0] = head;
    }

    pub fn grow(&mut self) {
        let last_block = self.body[self.body.len() - 1];
        let new_block = Position { x: last_block.x, y: last_block.y };
        self.body.push(new_block);
    }
}

fn new_tail_body() -> Vec<Position> {
    let tail: Vec<Position> = vec![Position { x: 2., y: 3. }, Position { x: 1., y: 3. }];
    return tail;
}
