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
            remaining: (0..7).map(|i| PlayerPiece::new(i)).collect(),
            queued: (0..7).map(|i| PlayerPiece::new(i)).collect()
        }
    }

    pub fn next(&mut self) -> PlayerPiece {
        match self.remaining.pop() {
            Some(piece) => piece,
            None => {
                self.remaining = (0..7).map(|i| PlayerPiece::new(i)).collect();
                std::mem::swap(&mut self.remaining, &mut self.queued);
                self.remaining.shuffle(&mut thread_rng());
                self.remaining.pop().unwrap()
            }
        }
    }

    pub fn peek(&self) -> &PlayerPiece {
        if self.remaining.len() != 0 {
            &self.remaining[0]
        } else {
            &self.queued[0]
        }
    }
}