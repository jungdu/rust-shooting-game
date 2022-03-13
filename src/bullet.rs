use piston_window::{types::Color, Context, G2d};

use crate::{draw::draw_rectangle, game_obj::GameObj, two_dimensional_space::Position};

const BULLET_COLOR: Color = [1.0, 0.0, 0.0, 1.0];
const BULLET_WIDTH: f64 = 5.0;
const BULLET_HEIGHT: f64 = 12.5;

pub trait Updatable {
    fn update(&mut self);
}

pub struct Bullet {
    position: Position,
    velocity: Position,
    destroyed: bool,
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
            destroyed: false,
        }
    }

    pub fn get_hit_box_points(&self) -> ((f64, f64), (f64, f64)) {
        (
            (
                self.position.x - (BULLET_WIDTH / 2.0),
                self.position.y - (BULLET_HEIGHT / 2.0),
            ),
            (
                self.position.x + (BULLET_WIDTH / 2.0),
                self.position.y + (BULLET_HEIGHT / 2.0),
            ),
        )
    }

    pub fn destroy(&mut self) {
        self.destroyed = true;
    }
}

impl GameObj for Bullet {
    fn update(&mut self) -> bool {
        self.position.add(&self.velocity);
        self.destroyed
    }

    fn draw(&self, con: &Context, g: &mut G2d) {
        draw_rectangle(
            BULLET_COLOR,
            self.position.x - (BULLET_WIDTH / 2.0),
            self.position.y - (BULLET_HEIGHT / 2.0),
            BULLET_WIDTH,
            BULLET_HEIGHT,
            con,
            g,
        )
    }
}
