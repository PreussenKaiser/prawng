use sdl2::{render::Canvas, video::Window, rect::{Point, Rect}};
use super::constants::{WINDOW_WIDTH, WINDOW_HEIGHT, PUCK_WIDTH, PUCK_HEIGHT};

pub fn draw_discriminator(canvas: &mut Canvas<Window>) -> Result<(), String> {
    let middle = WINDOW_WIDTH / 2;

    canvas.draw_line(
        Point::new(middle as i32, WINDOW_HEIGHT as i32),
        Point::new(middle as i32, 0))?;

    Ok(())
}

pub fn draw_rectangle(canvas: &mut Canvas<Window>, position: Point) -> Result<(), String> {
    let rectangle = Rect::new(
        position.x,
        position.y,
        PUCK_WIDTH,
        PUCK_HEIGHT);

    canvas.fill_rect(rectangle)?;

    Ok(())
}