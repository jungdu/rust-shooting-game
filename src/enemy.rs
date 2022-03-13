use piston_window::{types::Color, Context, G2d};

use crate::{draw::draw_rectangle, two_dimensional_space::Position};

const ENEMY_COLOR: Color = [0.7, 0.0, 0.7, 1.0];
const ENEMY_WIDTH: f64 = 50.0;
const ENEMY_HEIGHT: f64 = 25.0;

pub struct Enemy {
    position: Position,
}

impl Enemy {
    pub fn new(position_x: f64, position_y: f64) -> Enemy {
        Enemy {
            position: Position {
                x: position_x,
                y: position_y,
            },
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_rectangle(
            ENEMY_COLOR,
            self.position.x,
            self.position.y,
            ENEMY_WIDTH,
            ENEMY_HEIGHT,
            con,
            g,
        )
    }

    pub fn get_hit_box_points(&self) -> ((f64, f64), (f64, f64)) {
        (
            (self.position.x, self.position.y),
            (
                self.position.x + ENEMY_WIDTH,
                self.position.y + ENEMY_HEIGHT,
            ),
        )
    }
}

pub fn create_enemies(enemy_positions: &[Position]) -> Vec<Enemy> {
    let mut enemies: Vec<Enemy> = Vec::new();
    enemy_positions
        .iter()
        .for_each(|position| enemies.push(Enemy::new(position.x, position.y)));
    enemies
}
