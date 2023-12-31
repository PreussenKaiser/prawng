use sdl2::{video::Window, render::Canvas, rect::{Point, Rect}, pixels::Color};

use crate::{game::{Game, ball::Ball}, position::Position};

pub struct Renderer {
    canvas: Canvas<Window>,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Renderer { canvas })
    }

    pub fn draw(&mut self, game: &Game) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::GRAY);
        self.draw_discriminator()?;

        self.canvas.set_draw_color(Color::WHITE);
        self.draw_player(game.player1.rect().point())?;
        self.draw_player(game.player2.rect().point())?;
        self.draw_ball(game.ball)?;

        self.canvas.present();

        Ok(())
    }

    fn draw_discriminator(&mut self) -> Result<(), String> {
        let start = Point::new(400, 0);
        let end = Point::new(400, 600);

        self.canvas.draw_line(start, end)?;

        Ok(())
    }

    fn draw_player(&mut self, player_pos: Point) -> Result<(), String> {
        let rect = Rect::new(player_pos.x, player_pos.y, 16, 64);

        self.canvas.fill_rect(rect)?;

        Ok(())
    }

    fn draw_ball(&mut self, ball: Ball) -> Result<(), String> {
        self.canvas.fill_rect(ball.rect())?;

        Ok(())
    }
}