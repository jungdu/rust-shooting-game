use piston_window::{types::Color, Context, G2d, rectangle};

pub fn draw_rectangle(
  color: Color,
  x: f64,
  y: f64,
  width: f64,
  height: f64,
  con: &Context,
  g: &mut G2d
){
  let x = x;
  let y = y;

  rectangle(
    color,
    [
      x,
      y,
      width,
      height,
    ],
    con.transform,
    g,
  )
}