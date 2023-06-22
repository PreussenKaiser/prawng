use sdl2::rect::Point;

pub struct Ball {
    position: Point,
    size: u8,
}

impl Ball {
    pub fn new(position: Point, size: u8) -> Ball {
        return Ball { position, size, }
    }
}