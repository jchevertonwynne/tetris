extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::mouse::MouseButton;
use sdl2::rect::{Rect, Point};
use sdl2::render::WindowCanvas;

enum GuiState {
    Menu,
    Game
}

struct AppState {
    gui_state: GuiState,
    game_state: GameState,
}

struct PlayerPiece {
    tiles: Vec<Point>,
    stationary: bool,
}

struct GameState {
    tiles: [[bool; 20]; 10],
    turns: i64,
    active: Option<PlayerPiece>,
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

        canvas.set_draw_color(Color::RGB(128, 50, 200));
        match &self.active {
            Some(piece) => {
                for p in &piece.tiles {
                    canvas.fill_rect(Rect::new(p.x() * 40, p.y() * 40, 40, 40))?
                }
            },
            None => (),
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
                mousestate,
                x, y,
                ..
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
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                match &self.active {
                    Some(piece) => {
                        if piece.tiles.iter().all(|p| p.x() > 0) && piece.tiles.iter().all(|p| !self.tiles[(p.x() - 1) as usize][p.y() as usize]) {
                            self.active = Some(PlayerPiece{
                                tiles: piece.tiles.clone().iter().map(|p| p.offset(-1, 0)).collect(),
                                stationary: piece.stationary
                            })
                        }
                    }
                    None => ()
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                match &self.active {
                    Some(piece) => {
                        if piece.tiles.iter().all(|p| p.x() < (self.tiles.len() - 1) as i32)
                            && piece.tiles.iter().all(|p| !self.tiles[(p.x() + 1) as usize][p.y() as usize]) {
                            self.active = Some(PlayerPiece{
                                tiles: piece.tiles.clone().iter().map(|p| p.offset(1, 0)).collect(),
                                stationary: piece.stationary
                            })
                        }
                    }
                    None => ()
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                'loader: loop {
                    match &self.active {
                        Some(piece) => {
                            if piece.tiles.iter().all(|p| p.y() < (self.tiles[0].len() - 1) as i32)
                                && piece.tiles.iter().all(|p| !self.tiles[p.x() as usize][(p.y() + 1) as usize]) {
                                self.active = Some(PlayerPiece {
                                    tiles: piece.tiles.iter().map(|p| p.offset(0, 1)).collect(),
                                    stationary: piece.stationary
                                });
                            } else {
                                break 'loader
                            }
                        },
                        None => break 'loader,
                    }
                }
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                x, y,
                ..
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
        if self.turns % 10 == 0 {
            match &self.active {
                Some(piece) => {
                    if piece.tiles.iter().all(|t| t.y() < (self.tiles[0].len() - 1) as i32)
                        && piece.tiles.iter().all(|p| !self.tiles[p.x() as usize][(p.y() + 1) as usize]) {
                        self.active = Some(PlayerPiece{
                            tiles: piece.tiles.iter().map(|p| p.offset(0, 1)).collect(),
                            stationary: false
                        })
                    } else {
                        if piece.stationary {
                            for p in &piece.tiles {
                                self.tiles[p.x() as usize][p.y() as usize] = true;
                            }
                            self.active = None;
                        } else {
                            self.active = Some(PlayerPiece{
                                tiles: piece.tiles.clone(),
                                stationary: true
                            });
                        }
                    }
                }
                None => {
                    let options: Vec<usize> = self.tiles.iter()
                        .map(|col| col[0])
                        .enumerate()
                        .filter(|(_, v)| !*v)
                        .map(|(i, _)| i)
                        .collect();

                    if options.is_empty() {
                        panic!("no options")
                    }

                    self.active = Some(PlayerPiece{
                        tiles: vec![
                            Point::new(4, 0),
                            Point::new(5, 0),
                            Point::new(4, 1),
                            Point::new(5, 1),
                        ],
                        stationary: false
                    });
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

    let mut app_state = AppState {
        gui_state: GuiState::Menu,
        game_state: GameState {
            tiles: [[false; 20]; 10],
            turns: 0,
            active: None
        }
    };

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
