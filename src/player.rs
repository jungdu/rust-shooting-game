use piston_window::{types::Color, Context, G2d};

use crate::{
    bullet::Bullet, direction::Direction, draw::draw_rectangle, two_dimensional_space::Position,
};

const PLAYER_COLOR: Color = [0.0, 1.0, 1.0, 1.0];
const POSITION_X_RANGE: (f64, f64) = (50.0, 700.0);
const HEAD_WIDTH: f64 = 25.0;
const HEAD_HEIGHT: f64 = 17.5;
const WING_WIDTH: f64 = 17.5;
const WING_HEIGHT: f64 = 12.5;

pub struct Player {
    position: Position,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Player {
        Player {
            position: Position { x, y },
        }
    }

    pub fn draw(&self, c: &Context, g: &mut G2d) {
        draw_rectangle(
            PLAYER_COLOR,
            self.position.x - 12.5,
            self.position.y - 5.0,
            HEAD_WIDTH,
            HEAD_HEIGHT,
            &c,
            g,
        );
        // left wing
        draw_rectangle(
            PLAYER_COLOR,
            self.position.x - (HEAD_WIDTH / 2.0) - WING_WIDTH,
            self.position.y + 7.5,
            WING_WIDTH,
            WING_HEIGHT,
            &c,
            g,
        );
        // right wing
        draw_rectangle(
            PLAYER_COLOR,
            self.position.x + (HEAD_WIDTH / 2.0),
            self.position.y + 7.5,
            WING_WIDTH,
            WING_HEIGHT,
            &c,
            g,
        );

        draw_rectangle(
            [1.0, 0.0, 0.0, 1.0],
            self.position.x - 2.5,
            self.position.y - 2.5,
            5.0,
            5.0,
            &c,
            g,
        );
    }

    pub fn move_position(&mut self, direction: Direction) {
        let moving_distance = 25.0;
        match direction {
            Direction::Right => {
                let next_position_x = self.position.x + moving_distance;
                self.position.x = if next_position_x > POSITION_X_RANGE.1 {
                    POSITION_X_RANGE.1
                } else {
                    next_position_x
                }
            }
            Direction::Left => {
                let next_position_x = self.position.x - moving_distance;
                self.position.x = if next_position_x < POSITION_X_RANGE.0 {
                    POSITION_X_RANGE.0
                } else {
                    next_position_x
                };
            }
        }
    }

    pub fn fire(&self) -> Bullet {
        Bullet::new(self.position.x, self.position.y, 0.0, -2.0)
    }
}
