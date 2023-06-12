use draw::{constants::{WINDOW_WIDTH, WINDOW_HEIGHT, TITLE}, game_objects::{draw_rectangle, draw_discriminator}};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point};

mod draw;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::WHITE);

    let mut player_position = WINDOW_HEIGHT / 2;
    
    'running: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'running,
                
                Event::KeyDown {
                    keycode: Option::Some(Keycode::Down) | Option::Some(Keycode::S),
                    ..
                } => { 
                    player_position += 8;
                }
                
                _ => {}
            }
        }
        
        draw_discriminator(&mut canvas)?;
        draw_rectangle(&mut canvas, Point::new(WINDOW_WIDTH as i32 - 32, player_position as i32))?;
        
        canvas.present();
    }

    Ok(())
}