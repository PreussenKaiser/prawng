use sdl2::rect::{Rect, Point};

pub trait Position {
    fn point(self) -> Point;
}

impl Position for Rect {
    fn point(self) -> Point {
        let point = Point::new(self.x(), self.y());

        return point;
    }
}