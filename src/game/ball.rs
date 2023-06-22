use sdl2::rect::Point;

#[derive(Clone, Copy)]
pub struct Ball {
    position: Point,
    size: u8
}

impl Ball {
    pub fn new(position: Point, size: u8) -> Ball {
        return Ball { position, size }
    }

    pub fn get_position(self) -> Point {
        return self.position;
    }

    pub fn get_size(self) -> u8 {
        return self.size;
    }
}