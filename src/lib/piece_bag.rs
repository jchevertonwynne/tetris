use crate::lib::PlayerPiece;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct PieceBag {
    remaining: Vec<PlayerPiece>,
    queued: Vec<PlayerPiece>,
}

impl PieceBag {
    pub fn new() -> Self {
        PieceBag{
            remaining: {
                let mut tiles: Vec<_> = (0..7).map(|i| PlayerPiece::new(i)).collect();
                tiles.shuffle(&mut thread_rng());
                tiles
            },
            queued: {
                let mut tiles: Vec<_> = (0..7).map(|i| PlayerPiece::new(i)).collect();
                tiles.shuffle(&mut thread_rng());
                tiles
            },
        }
    }

    pub fn next(&mut self) -> PlayerPiece {
        match self.remaining.pop() {
            Some(piece) => piece,
            None => {
                self.remaining = (0..7).map(|i| PlayerPiece::new(i)).collect();
                self.remaining.shuffle(&mut thread_rng());
                std::mem::swap(&mut self.remaining, &mut self.queued);
                self.remaining.pop().unwrap()
            }
        }
    }

    pub fn peek(&self) -> &PlayerPiece {
        if self.remaining.len() != 0 {
            &self.remaining[self.remaining.len() - 1]
        } else {
            &self.queued[self.queued.len() - 1]
        }
    }
}