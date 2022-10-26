use piston_window::{Glyphs, Context, GfxDevice, G2d, rectangle};

use crate::{WIDTH, game::BLOCK_WIDTH, text_view::{TextView, FONT_SIZE}, snake::Position, HEIGHT};

pub struct HUD {
    score_text: TextView,
    glyphs: Glyphs,
}

impl HUD {
    pub fn new(glyphs: Glyphs) -> HUD {
        let score_text = new_score_text();
        HUD {
            score_text,
            glyphs,
        }
    }

    pub fn draw(&mut self, c: Context, g: &mut G2d, d: &mut GfxDevice) {
        self.score_text.draw(c, g, &mut self.glyphs);
        self.glyphs.factory.encoder.flush(d);
    }

    pub fn draw_death_screen(&mut self, c: Context, g: &mut G2d) {
        rectangle(
            [1., 0., 0., 0.4],
            [0., 0., WIDTH, HEIGHT],
            c.transform,
            g
        );

        let death_text = new_death_text();
        let death_subtitle = new_death_subtitle();

        death_text.draw(c, g, &mut self.glyphs);
        death_subtitle.draw(c, g, &mut self.glyphs);
    }

    pub fn update(&mut self, score: i8) {
        self.score_text.str = format!("{}", score);
    }
}

fn new_score_text() -> TextView {
    let pos = Position {
        x: WIDTH / 2. - 2.0,
        y: 3. * BLOCK_WIDTH,
    };
    TextView::new("0", pos)
}

fn new_death_text() -> TextView {
    let pos = Position {
        x: WIDTH / 2.,
        y: HEIGHT / 2.,
    };
    TextView::new("You fucked it.", pos)
}

fn new_death_subtitle() -> TextView {
    let pos = Position {
        x: WIDTH / 2.,
        y: (HEIGHT / 2.) + (FONT_SIZE as f64) + 10.,
    };
    TextView::new("Press [SPACE] to try again.", pos)
}