use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::lib::{PieceBag, PlayerPiece, Sounds};
use std::sync::mpsc::SyncSender;

pub struct GameState {
    tiles: [[bool; 20]; 10],
    turns: i64,
    active: Option<PlayerPiece>,
    bag: PieceBag,
    cleared: u64
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            tiles: [[false; 20]; 10],
            turns: 0,
            active: None,
            bag: PieceBag::new(),
            cleared: 0
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 0));

        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[0].len() {
                if self.tiles[i][j] {
                    canvas.fill_rect(Rect::new((i * 40) as i32, (j * 40) as i32, 40, 40))?;
                }
            }
        }

        match &self.active {
            Some(piece) => {
                piece.draw(canvas, &self.tiles)?;
            }
            None => (),
        }

        canvas.set_draw_color(Color::RGB(0, 0, 255));

        for i in 0..=self.tiles.len() {
            canvas.draw_line(Point::new((i * 40) as i32, 0), Point::new((i * 40) as i32, 800))?;
        }

        for j in 0..=self.tiles[0].len() {
            canvas.draw_line(Point::new(0, (j * 40) as i32), Point::new(400, (j * 40) as i32))?;
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_line(Point::new(0, 160), Point::new(400, 160))?;

        canvas.set_draw_color(Color::RGB(128, 50, 200));

        for square in self.bag.peek().base().iter() {
            canvas.fill_rect(Rect::new(500 + 50 * square.x(), 100 + 50 * square.y(), 50, 50))?;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 255));
        for i in 0..=4 {
            canvas.draw_line(Point::new(500, 100 + i * 50), Point::new(700, 100 + i * 50))?;
            canvas.draw_line(Point::new(500 + i * 50, 100), Point::new(500 + i * 50, 300))?;
        }

        Ok(())
    }

    pub fn handle(&mut self, event: Event) -> bool {
        match event {
            Event::KeyDown { keycode: Some(Keycode::A), .. }
            | Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                if let Some(piece) = &self.active {
                    if let Some(new_piece) = piece.go_left(&self.tiles) {
                        self.active = Some(new_piece);
                    }
                }
            }
            Event::KeyDown { keycode: Some(Keycode::D), .. }
            | Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                if let Some(piece) = &self.active {
                    if let Some(new_piece) = piece.go_right(&self.tiles) {
                        self.active = Some(new_piece);
                    }
                }
            }
            Event::KeyDown { keycode: Some(Keycode::S), .. }
            | Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                'loader: loop {
                    match &self.active {
                        Some(piece) => {
                            match piece.go_down(&self.tiles) {
                                Some(new_piece) => self.active = Some(new_piece),
                                None => break 'loader
                            }
                        }
                        None => break 'loader
                    }
                }
            }
            Event::KeyDown { keycode: Some(Keycode::R), .. }
            | Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                if let Some(p) = &self.active {
                    if let Some(new_p) = p.rotate(&self.tiles) {
                        self.active = Some(new_p);
                    }
                }
            }
            Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                match &self.active {
                    Some(piece) => {
                        self.active = Some(self.bag.swap(piece.clone()));
                    }
                    None => ()
                }
            }
            Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                let x = (x / 40) as usize;
                let y = (y / 40) as usize;
                if x < 10 {
                    self.tiles[x][y] = !self.tiles[x][y];
                }
            }
            Event::MouseMotion { mousestate, x, y, .. } => {
                if mousestate.left() {
                    let x = (x / 40) as usize;
                    let y = (y / 40) as usize;
                    if x < 10 {
                        self.tiles[x][y] = true;
                    }
                }
            }
            _ => ()
        }
        true
    }

    pub fn update(&mut self, audio: SyncSender<Sounds>) -> Option<u64> {
        let mut score = 0;
        let mut scalar = 1;

        if self.turns % (30 - (self.cleared / 3)) as i64 == 0 {
            for i in 0..self.tiles.len() {
                if (0..4).any(|j| self.tiles[i][j]) {
                    audio.send(Sounds::End).expect("send this pls :)");
                    return None;
                }
            }

            match &self.active {
                Some(piece) => {
                    if let Some(p) = piece.go_down(&self.tiles) {
                        self.active = Some(p)
                    } else {
                        if piece.is_stationary() {
                            for p in piece.get_tiles() {
                                self.tiles[p.x() as usize][p.y() as usize] = true;
                            }
                            self.active = None;
                            audio.send(Sounds::Ground).expect("you should always send");
                        } else {
                            self.active = Some(piece.set_stationary(true));
                        }
                    }
                }
                None => {
                    self.active = Some(self.bag.next());
                }
            }

            for j in 0..self.tiles[0].len() {
                if self.tiles.iter().all(|row| row[j]) {
                    score += scalar;
                    scalar += 1;
                    self.cleared += 1;
                    for i in 0..self.tiles.len() {
                        self.tiles[i][j] = false;
                    }
                    audio.send(Sounds::Clear).expect("should send sound");
                }
            }

            for j in (1..self.tiles[0].len()).rev() {
                if self.tiles.iter().all(|row| !row[j]) {
                    for i in 0..self.tiles.len() {
                        self.tiles[i][j] = self.tiles[i][j - 1];
                        self.tiles[i][j - 1] = false;
                    }
                }
            }
        }

        self.turns += 1;

        Some(score)
    }
}