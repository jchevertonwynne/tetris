use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;

pub struct PlayerPiece {
    tiles: Vec<Point>,
    stationary: bool,
}

impl PlayerPiece {
    pub fn new() -> PlayerPiece {
        PlayerPiece{
            tiles: vec![
                Point::new(4, 0),
                Point::new(5, 0),
                Point::new(4, 1),
                Point::new(5, 1),
            ],
            stationary: false
        }
    }

    pub fn lowest_possible_position(&self, board: &[[bool; 20]; 10]) -> PlayerPiece {
        let mut res = PlayerPiece{
            tiles: self.tiles.clone(),
            stationary: self.stationary,
        };
        while res.can_go_down(board) {
            res = res.go_down(board).unwrap()
        }
        res
    }

    pub fn tiles(&self) -> &Vec<Point> {
        &self.tiles
    }

    pub fn is_stationary(&self) -> bool {
        self.stationary
    }

    pub fn set_stationary(&self, s: bool) -> PlayerPiece {
        PlayerPiece {
            tiles: self.tiles.clone(),
            stationary: s,
        }
    }

    pub fn go_left(&self, board: &[[bool; 20]; 10]) -> Option<PlayerPiece> {
        if self.can_go_left(board) {
            Some(self.move_piece(-1, 0))
        } else {
            None
        }
    }

    fn can_go_left(&self, board: &[[bool; 20]; 10]) -> bool {
        self.tiles.iter().all(|p| p.x() > 0)
            && self.tiles.iter().all(|p| !board[(p.x() - 1) as usize][p.y() as usize])
    }

    pub fn go_right(&self, board: &[[bool; 20]; 10]) -> Option<PlayerPiece> {
        if self.can_go_right(board) {
            Some(self.move_piece(1, 0))
        } else {
            None
        }
    }

    fn can_go_right(&self, board: &[[bool; 20]; 10]) -> bool {
        self.tiles.iter().all(|p| p.x() < (board.len() - 1) as i32)
            && self.tiles.iter().all(|p| !board[(p.x() + 1) as usize][p.y() as usize])
    }

    pub fn go_down(&self, board: &[[bool; 20]; 10]) -> Option<PlayerPiece> {
        if self.can_go_down(board) {
            Some(self.move_piece(0, 1))
        } else {
            None
        }
    }

    fn can_go_down(&self, board: &[[bool; 20]; 10]) -> bool {
        self.tiles.iter().all(|p| p.y() < (board[0].len() - 1) as i32)
            && self.tiles.iter().all(|p| !board[p.x() as usize][(p.y() + 1) as usize])
    }

    pub fn move_piece(&self, x: i32, y: i32) -> PlayerPiece {
        PlayerPiece {
            tiles: self.tiles.clone().iter().map(|p| p.offset(x, y)).collect(),
            stationary: self.stationary
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, board: &[[bool; 20]; 10]) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(128, 50, 200));
        for p in &self.tiles {
            canvas.fill_rect(Rect::new(p.x() * 40, p.y() * 40, 40, 40))?
        }

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        for p in self.lowest_possible_position(board).tiles {
            canvas.draw_rect(Rect::new(p.x() * 40 + 1, p.y() * 40 + 1, 38, 38))?;
        }

        Ok(())
    }
}