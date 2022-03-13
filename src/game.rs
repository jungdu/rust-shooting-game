use piston_window::{Context, G2d, Key};

use crate::enemy::create_enemies;
use crate::game_obj::{draw_objs, update_objs};
use crate::two_dimensional_space::{detect_collision, Position};
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
            Key::Up | Key::Space => match self.player.fire() {
                Some(bullet) => self.bullets.push(bullet),
                None => {}
            },
            Key::Down => {}
            Key::Left => self.player.move_position(Direction::Left),
            Key::Right => self.player.move_position(Direction::Right),
            _ => {}
        };
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.player.draw(con, g);
        draw_objs(&self.bullets, con, g);
        draw_objs(&self.enemies, con, g);
    }

    pub fn update(&mut self) {
        update_objs(&mut self.bullets);
        update_objs(&mut self.enemies);
        self.process_collision();

        if self.enemies.len() == 0 {
            self.enemies = create_initial_enemies();
        }
    }

    fn process_collision(&mut self) {
        self.bullets.iter_mut().for_each(|bullet| {
            let bullet_hit_box_points = bullet.get_hit_box_points();
            self.enemies.iter_mut().for_each(|enemy| {
                let enemy_hit_box_points = enemy.get_hit_box_points();
                let result = detect_collision(
                    bullet_hit_box_points.0,
                    bullet_hit_box_points.1,
                    enemy_hit_box_points.0,
                    enemy_hit_box_points.1,
                );
                if result {
                    bullet.destroy();
                    enemy.reduce_hp(1);
                }
            })
        });
    }
}

fn create_initial_enemies() -> Vec<Enemy> {
    let enemy_positions = [
        Position { y: 100.0, x: 100.0 },
        Position { y: 100.0, x: 250.0 },
        Position { y: 100.0, x: 400.0 },
        Position { y: 100.0, x: 550.0 },
        Position { y: 140.0, x: 175.0 },
        Position { y: 140.0, x: 325.0 },
        Position { y: 140.0, x: 475.0 },
    ];
    create_enemies(&enemy_positions)
}
