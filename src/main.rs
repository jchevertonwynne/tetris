extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::mouse::MouseButton;
use sdl2::rect::{Rect, Point};
use sdl2::render::WindowCanvas;
use std::mem::swap;

struct Board {
    turns: i64,
    tiles: [[bool; 20]; 10]
}

impl Board {
    fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 0));
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[0].len() {
                if self.tiles[i][j] {
                    canvas.fill_rect(Rect::new((i * 40) as i32, (j * 40) as i32, 40, 40))?
                }
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 255));
        for i in 0..=self.tiles.len() {
            canvas.draw_line(Point::new((i * 40) as i32, 0), Point::new((i * 40) as i32, 800))?
        }

        for j in 0..=self.tiles[0].len() {
            canvas.draw_line(Point::new(0, (j * 40) as i32), Point::new(400, (j * 40) as i32))?
        }

        Ok(())
    }

    fn update(&mut self) {
        if self.turns % 30 == 0 {
            for i in 0..self.tiles.len() {
                for j in (1..self.tiles[0].len()).rev() {
                    if self.tiles[i][j - 1] && !self.tiles[i][j] {
                        self.tiles[i][j - 1] = false;
                        self.tiles[i][j] = true;
                    }
                }
            }
        }

        for j in 0..self.tiles[0].len() {
            let mut allGood = true;
            for i in 0..self.tiles.len() {
                if !self.tiles[i][j] {
                    allGood = false;
                    break;
                }
            }
            if allGood {
                for i in 0..self.tiles.len() {
                    self.tiles[i][j] = false;
                }
            }
        }

        self.turns += 1;
    }
}

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

    let mut canvas = window.into_canvas().build().unwrap();
    // canvas.set_scale(1, 1.0)?;

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame = 0;

    let mut state = Board { tiles: [[false; 20]; 10], turns: 0 };

    'running: loop {
        frame += 1;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown {
                    timestamp, window_id, which, mouse_btn, clicks, x, y
                } => {
                    match mouse_btn {
                        MouseButton::Left => {
                            let x = (x / 40) as usize;
                            let y = (y / 40) as usize;
                            if x < 10 {
                                let tile = &mut state.tiles[x][y];
                                *tile = !*tile;
                            }
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        state.update();
        state.draw(&mut canvas)?;

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
