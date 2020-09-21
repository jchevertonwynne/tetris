use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use rand::Rng;
use lazy_static::lazy_static;
use std::ops::Add;

lazy_static! {
    static ref OPTIONS: [PlayerPiece; 7] = [
        PlayerPiece{ // square
            anchor: Point::new(3, 0),
            box_size: 4,
            tiles: [
                Point::new(1, 1),
                Point::new(2, 1),
                Point::new(1, 2),
                Point::new(2, 2),
            ],
            stationary: false
        },
        PlayerPiece{ // T shape
            anchor: Point::new(3, 0),
            box_size: 3,
            tiles: [
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(2, 1),
                Point::new(1, 0),
            ],
            stationary: false
        },
        PlayerPiece{ // long
            anchor: Point::new(3, 0),
            box_size: 4,
            tiles: [
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(2, 1),
                Point::new(3, 1),
            ],
            stationary: false
        },
        PlayerPiece{ // S 1
            anchor: Point::new(3, 0),
            box_size: 3,
            tiles: [
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(1, 2),
                Point::new(2, 2),
            ],
            stationary: false
        },
        PlayerPiece{ // S 2
            anchor: Point::new(3, 0),
            box_size: 3,
            tiles: [
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(1, 0),
                Point::new(2, 0),
            ],
            stationary: false
        },
        PlayerPiece{ // L
            anchor: Point::new(3, 0),
            box_size: 3,
            tiles: [
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(2, 1),
                Point::new(2, 2),
            ],
            stationary: false
        },
        PlayerPiece{ // L
            anchor: Point::new(3, 0),
            box_size: 3,
            tiles: [
                Point::new(0, 1),
                Point::new(1, 1),
                Point::new(2, 1),
                Point::new(2, 0),
            ],
            stationary: false
        }
    ];
}

pub struct PlayerPiece {
    anchor: Point,
    box_size: usize,
    tiles: [Point; 4],
    stationary: bool,
}

impl Clone for PlayerPiece {
    fn clone(&self) -> Self {
        Self{
            anchor: self.anchor,
            box_size: self.box_size,
            tiles: self.tiles,
            stationary: self.stationary
        }
    }
}

impl PlayerPiece {
    pub fn new() -> PlayerPiece {
        let mut rng = rand::thread_rng();
        OPTIONS[rng.gen_range(0, OPTIONS.len())].clone()
    }

    pub fn try_rotate(&self,  board: &[[bool; 20]; 10]) -> Option<PlayerPiece> {
        let new = self.rotate();
        if new.get_tiles().iter().all(|t| t.x() > 0 && t.x() < board.len() as i32 && t.y() > 0 && t.y() < board[0].len() as i32 && !board[t.x() as usize][t.y() as usize]) {
            Some(new)
        } else {
            None
        }
    }

    fn rotate(&self) -> PlayerPiece {
        let mut pieces: [Point; 4] = [Point::new(0, 0); 4];
        for (i, p) in self.tiles.iter().enumerate() {
            pieces[i] = Point::new(self.box_size as i32 - 1 - p.y(), p.x());
        }
        let mut new_piece = self.clone();
        new_piece.tiles = pieces;
        new_piece
    }

    pub fn lowest_possible_position(&self, board: &[[bool; 20]; 10]) -> PlayerPiece {
        let mut res = self.clone();
        while let Some(b) = res.go_down(board) {
            res = b;
        }
        res
    }

    pub fn get_tiles(&self) -> Vec<Point> {
        self.tiles.iter()
            .map(|t| t.add(self.anchor))
            .collect()
    }

    pub fn is_stationary(&self) -> bool {
        self.stationary
    }

    pub fn set_stationary(&self, s: bool) -> PlayerPiece {
        let mut new_piece = self.clone();
        new_piece.stationary = s;
        new_piece
    }

    pub fn go_left(&self, board: &[[bool; 20]; 10]) -> Option<PlayerPiece> {
        if self.can_go_left(board) {
            Some(self.move_piece(-1, 0))
        } else {
            None
        }
    }

    fn can_go_left(&self, board: &[[bool; 20]; 10]) -> bool {
        self.get_tiles().iter().all(|p| p.x() > 0)
            && self.get_tiles().iter().all(|p| !board[(p.x() - 1) as usize][p.y() as usize])
    }

    pub fn go_right(&self, board: &[[bool; 20]; 10]) -> Option<PlayerPiece> {
        if self.can_go_right(board) {
            Some(self.move_piece(1, 0))
        } else {
            None
        }
    }

    fn can_go_right(&self, board: &[[bool; 20]; 10]) -> bool {
        self.get_tiles().iter().all(|p| p.x() < (board.len() - 1) as i32)
            && self.get_tiles().iter().all(|p| !board[(p.x() + 1) as usize][p.y() as usize])
    }

    pub fn go_down(&self, board: &[[bool; 20]; 10]) -> Option<PlayerPiece> {
        if self.can_go_down(board) {
            Some(self.move_piece(0, 1))
        } else {
            None
        }
    }

    fn can_go_down(&self, board: &[[bool; 20]; 10]) -> bool {
        self.get_tiles().iter().all(|p| p.y() < (board[0].len() - 1) as i32)
            && self.get_tiles().iter().all(|p| !board[p.x() as usize][(p.y() + 1) as usize])
    }

    pub fn move_piece(&self, x: i32, y: i32) -> PlayerPiece {
        let mut new_piece = self.clone();
        new_piece.anchor = new_piece.anchor.offset(x, y);
        new_piece
    }

    pub fn render(&self, canvas: &mut WindowCanvas, board: &[[bool; 20]; 10]) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB(128, 50, 200));
        for p in &self.get_tiles() {
            canvas.fill_rect(Rect::new(p.x() * 40, p.y() * 40, 40, 40))?
        }

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        for p in &self.lowest_possible_position(board).get_tiles() {
            canvas.draw_rect(Rect::new(p.x() * 40 + 1, p.y() * 40 + 1, 38, 38))?;
        }

        Ok(())
    }
}