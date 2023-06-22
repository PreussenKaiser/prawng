use game::Game;
use renderer::Renderer;
use sdl2::{event::Event, keyboard::Keycode, rect::Point};

mod game;
mod renderer;

const TITLE: &str = "PRAWNG";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

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
                Event::KeyDown { keycode: Some(keycoode), .. } => {
                    let player_pos = game.player1.get_position();
                    
                    match keycoode {
                        Keycode::Up => game.player1.move_position(Point::new(player_pos.x, player_pos.y - 8)),
                        Keycode::Down => game.player1.move_position(Point::new(player_pos.x, player_pos.y + 8)),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        renderer.draw(&game)?;
    }

    Ok(())
}