use piston_window::{Context, G2d, Key};

use crate::enemy::create_enemies;
use crate::two_dimensional_space::Position;
use crate::{bullet::Bullet, direction::Direction, enemy::Enemy, player::Player};

pub struct Game {
    player: Player,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player::new(375.0, 675.0),
            bullets: Vec::new(),
            enemies: create_initial_enemies(),
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        println!("Key pressed: {:?}", key);

        match key {
            Key::Up | Key::Space => {
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
        self.enemies.iter().for_each(|enemy| enemy.draw(con, g));
    }

    pub fn update(&mut self) {
        self.bullets.iter_mut().for_each(|bullet| bullet.update());
    }
}

fn create_initial_enemies() -> Vec<Enemy> {
    let enemy_positions = [
        Position { y: 100.0, x: 100.0 },
        Position { y: 100.0, x: 250.0 },
        Position { y: 100.0, x: 400.0 },
        Position { y: 100.0, x: 550.0 },
    ];
    create_enemies(&enemy_positions)
}
