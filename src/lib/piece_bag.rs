use crate::lib::PlayerPiece;

pub struct PieceBag {
    remaining: Vec<PlayerPiece>,
    queued: Vec<PlayerPiece>,
}

impl PieceBag {
    pub fn new() -> Self {
        PieceBag {
            remaining: PlayerPiece::all_shuffled(),
            queued: PlayerPiece::all_shuffled(),
        }
    }

    pub fn next(&mut self) -> PlayerPiece {
        match self.remaining.pop() {
            Some(piece) => piece,
            None => {
                std::mem::swap(&mut self.remaining, &mut self.queued);
                self.queued = PlayerPiece::all_shuffled();
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