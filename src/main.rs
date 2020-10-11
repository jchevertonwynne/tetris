extern crate sdl2;

use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::time::Duration;

use sdl2::mixer::AUDIO_S16LSB;
use sdl2::pixels::Color;

use crate::lib::{AppState, Sound};

mod lib;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let clear_sound = sdl2::mixer::Music::from_file(Path::new("sounds/clear.ogg"))?;
    let ground_sound = sdl2::mixer::Music::from_file(Path::new("sounds/ground.ogg"))?;
    let end_sound = sdl2::mixer::Music::from_file(Path::new("sounds/game_end.ogg"))?;

    let frequency = 44100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = 2; // Stereo
    let chunk_size = 1024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
    sdl2::mixer::allocate_channels(2);

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font(Path::new("DroidSansMono.ttf"), 64)?;

    let mut event_pump = sdl_context.event_pump()?;
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

    let mut app_state = AppState::new();

    let (a_send, a_recv): (SyncSender<Sound>, Receiver<Sound>) = mpsc::sync_channel(10);

    'running: loop {
        if !event_pump.poll_iter().all(|e| app_state.handle(e)) {
            break 'running;
        }

        app_state.update(a_send.clone());
        app_state.draw(&mut canvas, &font)?;

        while let Ok(s) = a_recv.recv_timeout(Duration::from_millis(1)) {
            match s {
                Sound::Clear => clear_sound.play(1)?,
                Sound::Ground => ground_sound.play(1)?,
                Sound::End => end_sound.play(1)?,
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
