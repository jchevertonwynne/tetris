use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::lib::GameState;
use std::path::Path;

#[derive(PartialEq)]
enum GuiState {
    Menu,
    Game,
    Lost
}

pub struct AppState {
    gui_state: GuiState,
    game_state: GameState,
    score: u64,
    high_score: u64
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            gui_state: GuiState::Menu,
            game_state: GameState::new(),
            score: 0,
            high_score: 0
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        self.game_state.draw(canvas)?;

        match self.gui_state {
            GuiState::Menu
            | GuiState::Lost => canvas.set_draw_color(Color::RGB(255, 0, 0)),
            GuiState::Game => canvas.set_draw_color(Color::RGB(0, 255, 0)),
        }
        canvas.fill_rect(Rect::new(500, 100, 200, 200))?;

        canvas.set_draw_color(Color::RGB(0, 255, 0));

        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let font = ttf_context.load_font(Path::new("DroidSansMono.ttf"), 128)?;

        let score = font.render(&*format!("Score: {}", self.score))
            .blended(Color::RGBA(255, 0, 0, 255)).map_err(|e| e.to_string())?;
        let high_score = font.render(&*format!("High score: {}", self.high_score))
            .blended(Color::RGBA(255, 0, 0, 255)).map_err(|e| e.to_string())?;

        let texture_creator = canvas.texture_creator();

        let score_texture = texture_creator.create_texture_from_surface(score).map_err(|e| e.to_string())?;
        let high_score_texture = texture_creator.create_texture_from_surface(high_score).map_err(|e| e.to_string())?;

        canvas.copy(&score_texture, None, Some(Rect::new(450, 400, 200, 100)))?;
        canvas.copy(&high_score_texture, None, Some(Rect::new(450, 500, 300, 100)))?;

        canvas.present();

        Ok(())
    }

    pub fn handle(&mut self, event: Event) -> bool {
        match event {
            Event::Quit { .. }
            | Event::KeyDown { keycode: Some(Keycode::Escape), .. }
            | Event::KeyDown { keycode: Some(Keycode::Q), .. }=> false,
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
            _ => {
                if self.gui_state == GuiState::Game {
                    self.game_state.handle(event)
                } else {
                    true
                }
            }
        }
    }

    pub fn update(&mut self) {
        match self.gui_state {
            GuiState::Menu
            | GuiState::Lost => (),
            GuiState::Game => {
                match self.game_state.update() {
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