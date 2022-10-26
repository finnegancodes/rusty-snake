mod game;
mod direction;
mod snake;
mod apple;
mod tail;
mod hud;
mod text_view;

use piston_window::*;

use crate::{game::Game};

pub const WIDTH: f64 = 500.;
pub const HEIGHT: f64 = 500.;

fn main() {
    println!("------ Snake clone by Finnegan ------");

    let mut window_settings = WindowSettings::new("Rusty Snake", [WIDTH, HEIGHT])
        .exit_on_esc(true);

    window_settings.set_vsync(true);

    let mut window: PistonWindow = window_settings.build().unwrap();

    window.set_max_fps(60);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .expect("Unable to load assets");

    let glyphs = window
        .load_font(assets.join("Arvo-Regular.ttf"))
        .expect("Unable to load font");

    let mut game = Game::new(glyphs);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, d| {
            game.draw(c, g, d);
        });

        event.update(|args| {
            game.update(args.dt);
        });
    }
}
