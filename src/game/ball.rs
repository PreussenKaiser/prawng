use sdl2::rect::{Point, Rect};

use crate::{WINDOW_HEIGHT, math::Reverse};

#[derive(Clone, Copy)]
pub struct Ball {
    position: Point,
    velocity: Point,
    size: u8
}

impl Ball {
    pub fn new(position: Point, velocity: Point, size: u8) -> Ball {
        return Ball { position, velocity, size }
    }

    pub fn move_position(&mut self, player1_rect: Rect, player2_rect: Rect) {
        let new_y = self.position.y + self.velocity.y;

        if self.rect().has_intersection(player1_rect) || self.rect().has_intersection(player2_rect) {
            self.velocity.x = self.velocity.x().reverse();
        }

        if new_y >= WINDOW_HEIGHT as i32 || new_y <= 0 {
            self.velocity.y = self.velocity.y().reverse();
        }

        self.position = Point::new(
            self.position.x + self.velocity.x,
            self.position.y + self.velocity.y
        )
    }

    pub fn rect(self) -> Rect {
        let rect = Rect::new(
            self.position.x(),
            self.position.y(),
            self.size as u32,
            self.size as u32
        );

        return rect;
    }
}