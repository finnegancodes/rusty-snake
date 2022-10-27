mod apple;
mod direction;
mod game;
mod hud;
mod snake;
mod tail;
mod text_view;

use std::env;

use piston_window::*;

use crate::game::Game;

pub const WIDTH: f64 = 500.;
pub const HEIGHT: f64 = 500.;
pub const MOVE_INTERVAL: f64 = 0.2;

fn main() {
    println!("------ Snake clone by Finnegan ------");
    println!("Use [W, A, S, D] to move, [SPACE] to pause, [ESC] to quit.");

    let move_interval = get_move_interval();

    let mut window_settings = WindowSettings::new("Rusty Snake", [WIDTH, HEIGHT]).exit_on_esc(true);

    window_settings.set_vsync(true);

    let mut window: PistonWindow = window_settings.build().unwrap();

    window.set_max_fps(60);

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .expect("Unable to load assets");

    let glyphs = window
        .load_font(assets.join("Arvo-Regular.ttf"))
        .expect("Unable to load font");

    let mut game = Game::new(glyphs, move_interval);

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

fn get_move_interval() -> f64 {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 { return MOVE_INTERVAL;} 

    let str = &args[1];
    let move_interval = str
        .to_string()
        .parse::<f64>()
        .expect("Not a valid move interval");

    return move_interval;
}
