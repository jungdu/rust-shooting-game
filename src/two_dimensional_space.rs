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

pub fn detect_collision(
    left_up_position_a: (f64, f64),
    right_bottom_position_a: (f64, f64),
    left_up_position_b: (f64, f64),
    right_bottom_position_b: (f64, f64),
) -> bool {
    left_up_position_a.0 <= right_bottom_position_b.0
        && right_bottom_position_a.0 > left_up_position_b.0
        && left_up_position_a.1 < right_bottom_position_b.1
        && right_bottom_position_a.1 > left_up_position_b.1
}

#[cfg(test)]
mod tests {
    use crate::two_dimensional_space::detect_collision;

    #[test]
    fn rectangles_collision() {
        // a가 b를 포함하는 케이스
        assert_eq!(
            detect_collision((0.0, 0.0), (10.0, 10.0), (5.0, 5.0), (6.0, 6.0),),
            true
        );

        // a의 좌측하단에 b가 걸쳐진 케이스
        assert_eq!(
            detect_collision((0.0, 0.0), (10.0, 10.0), (9.0, 9.0), (15.0, 15.0),),
            true
        );

        // 크로스 하는 케이스
        assert_eq!(
            detect_collision((10.0, 10.0), (20.0, 20.0), (5.0, 13.0), (30.0, 17.0),),
            true
        );

        // 안만나는 케이스
        assert_eq!(
            detect_collision((0.0, 0.0), (10.0, 10.0), (15.0, 15.0), (18.0, 18.0),),
            false
        );
    }
}
