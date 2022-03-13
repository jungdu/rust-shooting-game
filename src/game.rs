use piston_window::{Context, G2d, Key};

use crate::{bullet::Bullet, direction::Direction, player::Player};

pub struct Game {
    player: Player,
    bullets: Vec<Bullet>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::new(375.0, 675.0),
            bullets: Vec::new(),
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        println!("Key pressed: {:?}", key);

        match key {
            Key::Up => {
                self.bullets.push(self.player.fire());
            }
            Key::Down => {}
            Key::Left => self.player.move_position(Direction::Left),
            Key::Right => self.player.move_position(Direction::Right),
            _ => {}
        };
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.player.draw(con, g);
        self.bullets.iter().for_each(|bullet| bullet.draw(con, g));
    }

    pub fn update(&mut self) {
        self.bullets.iter_mut().for_each(|bullet| bullet.update());
    }
}
