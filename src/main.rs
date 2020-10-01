mod lib;

extern crate sdl2;

use sdl2::pixels::Color;
use std::time::Duration;
use crate::lib::AppState;
use std::path::Path;

fn main() -> Result<(), String>{
    let sdl_context = sdl2::init()?;
    let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("tetrust", 800, 800)
        .position_centered()
        .build()
        .or_else(|e| Err(e.to_string()))?;
    let mut canvas = window.into_canvas()
        .build()
        .or_else(|e| Err(e.to_string()))?;

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font(Path::new("DroidSansMono.ttf"), 64)?;

    let mut app_state = AppState::new(font);

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
