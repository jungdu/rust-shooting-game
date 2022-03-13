use piston_window::{Context, G2d};

use crate::{
    direction::Direction, draw::draw_rectangle, game_obj::GameObj, two_dimensional_space::Position,
};

const ENEMY_WIDTH: f64 = 50.0;
const ENEMY_HEIGHT: f64 = 25.0;
const PATROL_RANGE: f64 = 120.0;
const PATROL_SPEED: f64 = 0.4;
const MAX_HP: usize = 3;

pub struct Enemy {
    initial_position: Position,
    moving_direction: Direction,
    position: Position,
    hp: usize,
    destroyed: bool,
}

impl Enemy {
    pub fn new(position_x: f64, position_y: f64) -> Enemy {
        let initial_position = Position {
            x: position_x,
            y: position_y,
        };

        Enemy {
            initial_position: initial_position,
            position: initial_position.clone(),
            destroyed: false,
            hp: MAX_HP,
            moving_direction: Direction::Right,
        }
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

    pub fn reduce_hp(&mut self, power: usize) {
        self.hp -= power;
        if self.hp <= 0 {
            self.destroyed = true;
        }
    }

    pub fn patrol(&mut self) {
        match self.moving_direction {
            Direction::Left => {
                let next_position_x = self.position.x - PATROL_SPEED;
                let min_position_x = self.initial_position.x - PATROL_RANGE;
                if next_position_x < min_position_x {
                    self.moving_direction = Direction::Right;
                    self.position.x = min_position_x;
                } else {
                    self.position.x = next_position_x;
                }
            }
            Direction::Right => {
                let next_position_x = self.position.x + PATROL_SPEED;
                let max_position_x = self.initial_position.x + PATROL_RANGE;
                if next_position_x > self.initial_position.x + PATROL_RANGE {
                    self.moving_direction = Direction::Left;
                    self.position.x = max_position_x;
                } else {
                    self.position.x = next_position_x;
                }
            }
        }
    }
}

impl GameObj for Enemy {
    fn update(&mut self) -> bool {
        self.patrol();
        self.destroyed
    }

    fn draw(&self, con: &Context, g: &mut G2d) {
        draw_rectangle(
            [0.7, 0.0, 0.7, ((self.hp as f32) / (MAX_HP as f32))],
            self.position.x,
            self.position.y,
            ENEMY_WIDTH,
            ENEMY_HEIGHT,
            con,
            g,
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
