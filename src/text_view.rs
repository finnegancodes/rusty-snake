use piston_window::{color::WHITE, Context, G2d, Glyphs, Text, Transformed};

use crate::{snake::Position};

pub const FONT_SIZE: u32 = 18;

pub struct TextView {
    pub str: String,
    pos: Position,
}

impl TextView {
    pub fn new<'a>(str: &'a str, pos: Position) -> TextView {
        TextView {
            str: str.to_string(),
            pos,
        }
    }

    pub fn draw(&self, c: Context, g: &mut G2d, glyphs: &mut Glyphs) {
        let font_size = FONT_SIZE as f64;
        let real_x = self.pos.x - ((self.str.len() as f64) * (font_size/4.));
        let transform = c.transform.trans(real_x, self.pos.y);
        Text::new_color(WHITE, FONT_SIZE)
            .draw(self.str.as_str(), glyphs, &c.draw_state, transform, g)
            .unwrap();
    }
}
