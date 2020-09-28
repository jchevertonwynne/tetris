use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
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
        PlayerPiece{ // L 1
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
        PlayerPiece{ // L 2
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
    pub fn new(i: usize) -> PlayerPiece {
        OPTIONS[i].clone()
    }

    pub fn rotate(&self, board: &[[bool; 20]; 10]) -> Option<PlayerPiece> {
        let new = self.try_rotate();
        if new.legal(board) {
            return Some(new)
        }
        else if self.anchor.x() < 0 {
            if let Some(piece_right) = self.go_right(board).map(|p| p.try_rotate()) {
                if piece_right.legal(board) {
                    return Some(piece_right)
                }
            }
        }
        else if self.anchor.x + self.box_size as i32 >= board.len() as i32 {
            if let Some(piece_left) = self.go_left(board).map(|p| p.try_rotate()) {
                if piece_left.legal(board) {
                    return Some(piece_left)
                }
            }
        }
        None
    }

    fn try_rotate(&self) -> PlayerPiece {
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
        let left = self.move_piece(-1, 0);
        if left.legal(board) {
            Some(left)
        } else {
            None
        }
    }

    pub fn go_right(&self, board: &[[bool; 20]; 10]) -> Option<PlayerPiece> {
        let right = self.move_piece(1, 0);
        if right.legal(board) {
            Some(right)
        } else {
            None
        }
    }

    pub fn go_down(&self, board: &[[bool; 20]; 10]) -> Option<PlayerPiece> {
        let down = self.move_piece(0, 1);
        if down.legal(board) {
            Some(down)
        } else {
            None
        }
    }

    fn move_piece(&self, x: i32, y: i32) -> PlayerPiece {
        let mut new_piece = self.clone();
        new_piece.anchor = new_piece.anchor.offset(x, y);
        new_piece
    }

    fn legal(&self, board: &[[bool; 20]; 10]) -> bool {
        self.get_tiles().iter().all(|t| t.x() >= 0 && t.x() < board.len() as i32 && t.y() >= 0 && t.y() < board[0].len() as i32
            && !board[t.x() as usize][t.y() as usize])
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