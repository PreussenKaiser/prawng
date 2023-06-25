use sdl2::rect::{Point, Rect};

use crate::WINDOW_HEIGHT;

#[derive(Clone, Copy)]
pub struct Paddle {
    rect: Rect,
}

impl Paddle {
    pub fn new(rect: Rect) -> Paddle {
        return Paddle { rect }
    }

    pub fn rect(self) -> Rect {
        return self.rect;
    }

    pub fn move_position(&mut self, position: Point) {
        let y = position.y;

        if y >= WINDOW_HEIGHT as i32 || y <= 0 {
            return;
        }

        self.rect.set_x(position.x());
        self.rect.set_y(position.y());
    }
}

#[cfg(test)]
mod tests {
    use sdl2::rect::{Point, Rect};
    use crate::{game::paddle::Paddle, position::Position};

    #[test]
    fn cannot_move_above() {
        let new_pos = Point::new(400, 600);
        let mut paddle = Paddle::new(Rect::new(400, 300, 0, 0));

        paddle.move_position(new_pos);
        let cur_pos = paddle.rect().point();

        assert_ne!(new_pos, cur_pos);
    }

    #[test]
    fn can_move_within_bounds() {
        let new_pos = Point::new(400, 308);
        let mut paddle = Paddle::new(Rect::new(400, 300, 0, 0));

        paddle.move_position(new_pos);
        let cur_pos = paddle.rect().point();

        assert_eq!(new_pos, cur_pos);
    }

    #[test]
    fn cannot_move_below() {
        let new_pos = Point::new(400, 0);
        let mut paddle = Paddle::new(Rect::new(400, 300, 0, 0));

        paddle.move_position(new_pos);
        let cur_pos = paddle.rect().point();

        assert_ne!(new_pos, cur_pos);
    }
}