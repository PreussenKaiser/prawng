use sdl2::rect::Point;

use crate::WINDOW_HEIGHT;

#[derive(Clone, Copy)]
pub struct Paddle {
    position: Point,
}

impl Paddle {
    pub fn new(position: Point) -> Paddle {
        return Paddle { position }
    }

    pub fn get_position(self) -> Point {
        return self.position;
    }

    pub fn move_position(&mut self, position: Point) {
        let y = position.y;

        if y >= WINDOW_HEIGHT as i32 || y <= 0 {
            return;
        }

        self.position = position;
    }
}

#[cfg(test)]
mod tests {
    use sdl2::rect::Point;
    use crate::game::paddle::Paddle;

    #[test]
    fn cannot_move_above() {
        let original_pos = Point::new(400, 300);
        let new_pos = Point::new(400, 600);
        let mut paddle = Paddle::new(original_pos);

        paddle.move_position(new_pos);

        assert_eq!(
            original_pos,
            paddle.get_position()
        );
    }

    #[test]
    fn can_move_within_bounds() {
        let original_pos = Point::new(400, 300);
        let new_pos = Point::new(400, 308);
        let mut paddle = Paddle::new(original_pos);

        paddle.move_position(new_pos);

        assert_ne!(
            original_pos,
            paddle.get_position()
        );
    }

    #[test]
    fn cannot_move_below() {
        let original_pos = Point::new(400, 300);
        let new_pos = Point::new(400, 0);
        let mut paddle = Paddle::new(original_pos);

        paddle.move_position(new_pos);

        assert_eq!(
            original_pos,
            paddle.get_position()
        );
    }
}