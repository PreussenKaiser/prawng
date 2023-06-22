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