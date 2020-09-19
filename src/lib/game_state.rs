use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use crate::lib::PlayerPiece;

pub struct GameState {
    tiles: [[bool; 20]; 10],
    turns: i64,
    active: Option<PlayerPiece>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            tiles: [[false; 20]; 10],
            turns: 0,
            active: None,
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(255, 255, 0));

        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[0].len() {
                if self.tiles[i][j] {
                    canvas.fill_rect(Rect::new((i * 40) as i32, (j * 40) as i32, 40, 40))?
                }
            }
        }

        match &self.active {
            Some(piece) => {
                piece.render(canvas)?;
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

    pub fn handle(&mut self, event: Event) -> bool {
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
            Event::KeyDown { keycode: Some(Keycode::A), .. } | Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                if let Some(piece) = &self.active {
                    if let Some(new_piece) = piece.go_left(&self.tiles) {
                        self.active = Some(new_piece);
                    }
                }
            }
            Event::KeyDown { keycode: Some(Keycode::D), .. } | Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                if let Some(piece) = &self.active {
                    if let Some(new_piece) = piece.go_right(&self.tiles) {
                        self.active = Some(new_piece);
                    }
                }
            }
            Event::KeyDown { keycode: Some(Keycode::S), .. } | Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                'loader: loop {
                    if let Some(piece) = &self.active {
                        if let Some(new_piece) = piece.go_down(&self.tiles) {
                            self.active = Some(new_piece);
                        } else {
                            break 'loader
                        }
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

    pub fn update(&mut self) {
        if self.turns % 10 == 0 {
            match &self.active {
                Some(piece) => {
                    if let Some(p) = piece.go_down(&self.tiles) {
                        self.active = Some(p)
                    } else {
                        if piece.is_stationary() {
                            for p in piece.tiles() {
                                self.tiles[p.x() as usize][p.y() as usize] = true;
                            }
                            self.active = None;
                        } else {
                            self.active = Some(piece.set_stationary(true));
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

                    self.active = Some(PlayerPiece::new());
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