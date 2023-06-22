use sdl2::rect::Point;

use crate::WINDOW_HEIGHT;

#[derive(Clone, Copy)]
pub struct Paddle {
    position: Point,
    is_player: bool, // Tech debt?
}

impl Paddle {
    pub fn new(position: Point, is_player: bool) -> Paddle {
        return Paddle { position, is_player, }
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