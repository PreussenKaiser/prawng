use sdl2::rect::Point;

use self::{paddle::Paddle, ball::Ball};

mod paddle;
mod ball;

pub struct Game {
    pub player1: Paddle,
    player2: Paddle,
    ball: Ball,
}

impl Game {
    pub fn new() -> Game {
        return Game {
            player1: Paddle::new(Point::new(768, 300), true),
            player2: Paddle::new(Point::new(32, 300), false),
            ball: Ball::new(Point::new(400, 300), 4),
        }
    }
}