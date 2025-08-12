
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
use sdl3::pixels::Color;
use sdl3::event::Event;
//use sdl3::rect::Rect;
use sdl3::render::FRect;
//use std::time::Duration;

const WIDTH:  usize = 64;
const HEIGHT: usize = 32;
type Display = [[bool; WIDTH]; HEIGHT];
const SCALE: u32 = 10; // scale of the "pixels" onscreen

fn main() -> Result<(), String> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("CHIP-8 Display", WIDTH as u32 * SCALE, HEIGHT as u32 * SCALE)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    // Example display: alternate pixels on/off
    let mut display: Display = [[false; WIDTH]; HEIGHT];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            display[y][x] = (x + y) % 2 == 0;
        }
    }

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // Draw frame
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        for (y, row) in display.iter().enumerate() {
            for (x, &pixel_on) in row.iter().enumerate() {
                if pixel_on {
                    canvas.set_draw_color(Color::WHITE);
                } else {
                    canvas.set_draw_color(Color::BLACK);
                }
                let _ = canvas.fill_rect(Some(FRect::new(
                    (x as f32) * SCALE as f32,
                    (y as f32) * SCALE as f32,
                    SCALE as f32,
                    SCALE as f32,
                )));
            }
        }

        canvas.present();
    }

    Ok(())
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1,2),3);
    }
    
}
