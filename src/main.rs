use std::time::Duration;

use game::Game;
use renderer::Renderer;
use sdl2::{event::Event, keyboard::Keycode, rect::Point};

mod game;
mod renderer;
mod math;
mod position;

const TITLE: &str = "PRAWNG";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const GAME_SPEED: i32 = 8;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window)?;
    let mut game = Game::new();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    let player1_pos = game.player1.rect();
                    let player2_pos = game.player2.rect();
                    
                    match keycode {
                        Keycode::Up => game.player1.move_position(Point::new(player1_pos.x, player1_pos.y - GAME_SPEED)),
                        Keycode::Down => game.player1.move_position(Point::new(player1_pos.x, player1_pos.y + GAME_SPEED)),
                        Keycode::W => game.player2.move_position(Point::new(player2_pos.x, player2_pos.y - GAME_SPEED)),
                        Keycode::S => game.player2.move_position(Point::new(player2_pos.x, player2_pos.y + GAME_SPEED)),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        game.ball.move_position(game.player1.rect(), game.player2.rect());
        renderer.draw(&game)?;
    }

    Ok(())
}