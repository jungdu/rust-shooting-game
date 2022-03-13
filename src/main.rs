use game::Game;
use piston_window::{clear, types::Color, Button, PistonWindow, PressEvent, WindowSettings};

mod bullet;
mod direction;
mod draw;
mod enemy;
mod game;
mod game_obj;
mod player;
mod two_dimensional_space;

const BACK_COLOR: Color = [0.2, 0.2, 0.2, 1.0];

fn main() {
    let (width, height) = (750.0, 750.0);

    let mut window: PistonWindow = WindowSettings::new("moving-rectangle", [width, height])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        game.update();

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });
    }
}
