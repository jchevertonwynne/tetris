mod lib;

extern crate sdl2;

use sdl2::pixels::Color;
use std::time::Duration;
use crate::lib::AppState;

fn main() -> Result<(), String>{
    let context = sdl2::init();

    let sdl_context = match context {
        Ok(result) => result,
        Err(message) => {
            println!("SDL reported error: '{}'", message);
            return Ok(());
        }
    };

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("tetris", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut app_state = AppState::new();

    'running: loop {
        if !event_pump.poll_iter().all(|e| app_state.handle(e)) {
            break 'running
        }

        app_state.update();
        app_state.draw(&mut canvas)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
