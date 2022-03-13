use piston_window::{Context, G2d};

use crate::two_dimensional_space::detect_collision;

pub trait GameObj {
    fn update(&mut self) -> bool;
    fn draw(&self, con: &Context, g: &mut G2d);
}

pub fn update_objs<T: GameObj>(game_objs: &mut Vec<T>) {
    let mut destroyed_obj_indexes: Vec<usize> = Vec::new();

    for (idx, game_obj) in game_objs.iter_mut().enumerate() {
        if game_obj.update() {
            destroyed_obj_indexes.push(idx);
        }
    }

    destroyed_obj_indexes.iter().for_each(|idx| {
        game_objs.remove(*idx);
    });
}

pub fn draw_objs<T: GameObj>(game_objs: &[T], con: &Context, g: &mut G2d) {
    game_objs.iter().for_each(|obj| obj.draw(con, g));
}

pub trait HitBox {
    fn get_hit_box_points(&self) -> ((f64, f64), (f64, f64));
}

pub fn detect_object_collision<T: HitBox, U: HitBox>(a: &T, b: &U) -> bool {
    let hit_box_position_a = a.get_hit_box_points();
    let hit_box_position_b = b.get_hit_box_points();

    detect_collision(
        hit_box_position_a.0,
        hit_box_position_a.1,
        hit_box_position_b.0,
        hit_box_position_b.1,
    )
}
