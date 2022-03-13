pub struct Position {
  pub x: f64,
  pub y: f64,
}

impl Position {
    pub fn add(&mut self, other: &Position) {
      self.x += other.x;
      self.y += other.y;
    }
}