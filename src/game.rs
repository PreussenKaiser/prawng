use sdl2::rect::Point;

use self::{paddle::Paddle, ball::Ball};

mod paddle;
pub mod ball;

pub struct Game {
    pub player1: Paddle,
    pub player2: Paddle,
    pub ball: Ball
}

impl Game {
    pub fn new() -> Game {
        return Game {
            player1: Paddle::new(Point::new(768, 300)),
            player2: Paddle::new(Point::new(32, 300)),
            ball: Ball::new(Point::new(400, 300), 12)
        }
    }
}