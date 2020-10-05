extern crate sdl2;

use std::path::Path;
use std::time::Duration;

use sdl2::pixels::Color;

use crate::lib::{AppState};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use sdl2::mixer::AUDIO_S16LSB;

mod lib;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let _audio = sdl_context.audio()?;
    let _mixer_context = sdl2::mixer::init(sdl2::mixer::InitFlag::OGG)?;
    let music = sdl2::mixer::Music::from_file(Path::new("sounds/clear.ogg"))?;

    let frequency = 44100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = 2; // Stereo
    let chunk_size = 1024;
    let _ = sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
    sdl2::mixer::allocate_channels(0);

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font(Path::new("DroidSansMono.ttf"), 64)?;

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

    canvas.set_scale(2.0, 2.0)?;
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut app_state = AppState::new(font);

    let (a_send, a_recv): (SyncSender<bool>, Receiver<bool>) = mpsc::sync_channel(10);

    'running: loop {
        if !event_pump.poll_iter().all(|e| app_state.handle(e)) {
            break 'running;
        }

        app_state.update(a_send.clone());
        app_state.draw(&mut canvas)?;

        if let Ok(_sound) = a_recv.recv_timeout(Duration::from_millis(1)) {
            music.play(1)?;
        } else {
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    Ok(())
}
