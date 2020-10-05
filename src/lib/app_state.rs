use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::ttf::Font;

use crate::lib::{GameState};
use std::sync::mpsc::SyncSender;

#[derive(PartialEq)]
enum GuiState {
    Menu,
    Game,
    Lost,
}

pub struct AppState<'a> {
    gui_state: GuiState,
    game_state: GameState,
    score: u64,
    high_score: u64,
    font: Font<'a, 'static>,
}

impl AppState<'_> {
    pub fn new<'a>(font: Font<'a, 'static>) -> AppState<'a> {
        AppState {
            gui_state: GuiState::Menu,
            game_state: GameState::new(),
            score: 0,
            high_score: 0,
            font,
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        self.game_state.draw(canvas)?;
        self.draw_pause_state(canvas)?;
        self.draw_scores(canvas)?;
        self.draw_next(canvas)?;

        canvas.present();

        Ok(())
    }

    pub fn draw_pause_state(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        match self.gui_state {
            GuiState::Menu
            | GuiState::Lost => canvas.set_draw_color(Color::RGB(255, 0, 0)),
            GuiState::Game => canvas.set_draw_color(Color::RGB(0, 255, 0)),
        }
        canvas.fill_rect(Rect::new(750, 0, 50, 50))
    }

    pub fn draw_scores(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 255, 0));

        let score = self.font.render(&*format!("Score: {}", self.score))
            .blended(Color::RGBA(255, 0, 0, 255)).map_err(|e| e.to_string())?;
        let high_score = self.font.render(&*format!("High score: {}", self.high_score))
            .blended(Color::RGBA(255, 0, 0, 255)).map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();

        let score_texture = texture_creator.create_texture_from_surface(score).map_err(|e| e.to_string())?;
        let high_score_texture = texture_creator.create_texture_from_surface(high_score).map_err(|e| e.to_string())?;

        canvas.copy(&score_texture, None, Some(Rect::new(450, 400, 200, 100)))?;
        canvas.copy(&high_score_texture, None, Some(Rect::new(450, 500, 300, 100)))
    }

    pub fn draw_next(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(128, 50, 200));

        for square in self.game_state.peek_next().base().iter() {
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
            Event::Quit { .. }
            | Event::KeyDown { keycode: Some(Keycode::Escape), .. }
            | Event::KeyDown { keycode: Some(Keycode::Q), .. } => false,
            Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                match &self.gui_state {
                    GuiState::Menu => self.gui_state = GuiState::Game,
                    GuiState::Game => self.gui_state = GuiState::Menu,
                    GuiState::Lost => {
                        self.game_state = GameState::new();
                        self.score = 0;
                        self.gui_state = GuiState::Game
                    }
                }
                true
            }
            Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                match &self.gui_state {
                    GuiState::Menu => self.gui_state = GuiState::Game,
                    GuiState::Game => {
                        return self.game_state.handle(event);
                    }
                    _ => ()
                };
                true
            }
            _ => {
                if self.gui_state == GuiState::Game {
                    self.game_state.handle(event)
                } else {
                    true
                }
            }
        }
    }

    pub fn update(&mut self, audio: SyncSender<bool>) {
        match self.gui_state {
            GuiState::Menu
            | GuiState::Lost => (),
            GuiState::Game => {
                match self.game_state.update(audio) {
                    Some(s) => {
                        self.score += s;
                    }
                    None => {
                        self.gui_state = GuiState::Lost;
                        if self.score > self.high_score {
                            self.high_score = self.score;
                        }
                    }
                }
            }
        }
    }
}