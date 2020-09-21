use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::lib::GameState;


#[derive(PartialEq)]
enum GuiState {
    Menu,
    Game,
}

pub struct AppState {
    gui_state: GuiState,
    game_state: GameState,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            gui_state: GuiState::Menu,
            game_state: GameState::new(),
        }
    }

    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
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

    pub fn handle(&mut self, event: Event) -> bool {
        match event {
            Event::Quit { .. }
            | Event::KeyDown { keycode: Some(Keycode::Escape), .. }
            | Event::KeyDown { keycode: Some(Keycode::Q), .. }=> false,
            Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                match &self.gui_state {
                    GuiState::Menu => self.gui_state = GuiState::Game,
                    GuiState::Game => self.gui_state = GuiState::Menu,
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
            GuiState::Menu => (),
            GuiState::Game => self.game_state.update(),
        }
    }
}