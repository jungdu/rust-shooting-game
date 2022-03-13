use piston_window::{types::Color, Context, G2d};

use crate::{draw::draw_rectangle, two_dimensional_space::Position};

const BULLET_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
const BULLET_WIDTH: f64 = 5.0;
const BULLET_HEIGHT: f64 = 12.5;

pub trait Updatable {
    fn update(&mut self);
}

pub struct Bullet {
    position: Position,
    velocity: Position,
}

impl Bullet {
    pub fn new(position_x: f64, position_y: f64, velocity_x: f64, velocity_y: f64) -> Bullet {
        Bullet {
            position: Position {
                x: position_x,
                y: position_y,
            },
            velocity: Position {
                x: velocity_x,
                y: velocity_y,
            },
        }
    }

    pub fn update(&mut self) {
        self.position.add(&self.velocity);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        draw_rectangle(
            BULLET_COLOR,
            self.position.x,
            self.position.y,
            BULLET_WIDTH,
            BULLET_HEIGHT,
            con,
            g,
        )
    }
}
