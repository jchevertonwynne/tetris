extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::mouse::MouseButton;
use sdl2::rect::{Rect, Point};
use sdl2::render::WindowCanvas;
use std::mem::swap;

enum GuiState {
    Menu,
    Game
}

type Tile = Vec<Point>;

struct AppState {
    gui_state: GuiState,
    game_state: GameState,
}

struct GameState {
    tiles: [[bool; 20]; 10],
    turns: i64,
    active: Option<Tile>,
}

impl AppState {
    fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        self.game_state.draw(canvas)?;

        match self.gui_state {
            GuiState::Menu => canvas.set_draw_color(Color::RGB(255, 0, 0)),
            GuiState::Game => canvas.set_draw_color(Color::RGB(0, 255, 0)),
        }
        canvas.fill_rect(Rect::new(500, 100, 200, 200))?;

        canvas.present();

        Ok(())
    }

    fn handle(&mut self, event: Event) -> bool {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => false,
            Event::KeyDown { keycode: Some(Keycode::Space),
                ..
            } => {
                match &self.gui_state {
                    GuiState::Menu => self.gui_state = GuiState::Game,
                    GuiState::Game => self.gui_state = GuiState::Menu,
                }
                true
            }
            _ => self.game_state.handle(event)
        }
    }

    fn update(&mut self) {
        match self.gui_state {
            GuiState::Menu => (),
            GuiState::Game => self.game_state.update(),
        }
    }
}

impl GameState {
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

    fn handle(&mut self, event: Event) -> bool {
        match event {
            Event::MouseMotion {
                timestamp, window_id, which, mousestate, x, y, xrel, yrel
            } => {
                if mousestate.left() {
                    let x = (x / 40) as usize;
                    let y = (y / 40) as usize;
                    if x < 10 {
                        if !self.tiles[x][y] {
                            self.tiles[x][y] = true;
                        }
                    }
                }
            }
            Event::MouseButtonDown {
                timestamp, window_id, which, mouse_btn: MouseButton::Left, clicks, x, y
            } => {
                let x = (x / 40) as usize;
                let y = (y / 40) as usize;
                if x < 10 {
                    let tile = &mut self.tiles[x][y];
                    *tile = !*tile;
                }
            }
            _ => ()
        }
        true
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

            for j in 0..self.tiles[0].len() {
                if self.tiles.iter().map(|row| row[j]).all(|v| v) {
                    for i in 0..self.tiles.len() {
                        self.tiles[i][j] = false;
                    }
                }
            }

            for j in (1..self.tiles[0].len()).rev() {
                if self.tiles.iter().map(|row| row[j]).all(|v| !v) {
                    for i in 0..self.tiles.len() {
                        self.tiles[i][j] = self.tiles[i][j - 1];
                        self.tiles[i][j - 1] = false;
                    }
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

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut frame = 0;

    let mut app_state = AppState {
        gui_state: GuiState::Menu,
        game_state: GameState {
            tiles: [[false; 20]; 10],
            turns: 0,
            active: None
        }
    };

    'running: loop {
        frame += 1;

        for event in event_pump.poll_iter() {
            if !app_state.handle(event) {
                break 'running
            }
        }

        app_state.update();
        app_state.draw(&mut canvas)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
