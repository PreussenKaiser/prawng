use sdl2::rect::{Point, Rect};

use crate::GAME_SPEED;

use self::{paddle::Paddle, ball::{Ball}};

pub mod ball;
mod paddle;

pub struct Game {
    pub player1: Paddle,
    pub player2: Paddle,
    pub ball: Ball
}

impl Game {
    pub fn new() -> Game {
        let ball_speed = GAME_SPEED / 2;
        let paddle_size = Point::new(16, 64);

        return Game {
            player1: Paddle::new(Rect::new(768, 300, paddle_size.x() as u32, paddle_size.y() as u32)),
            player2: Paddle::new(Rect::new(32, 300, paddle_size.x() as u32, paddle_size.y() as u32)),
            ball: Ball::new(
                Point::new(400, 300),
                Point::new(ball_speed, ball_speed),
                12
            )
        }
    }
}