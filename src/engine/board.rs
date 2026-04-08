use crate::engine::core::{Piece, Pos2d};

#[derive(Debug)]
pub struct Board {
    pub board: [Piece; 64],
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: [Piece::None; 64],
        }
    }
    pub fn set_at(&mut self, pos: &Pos2d, piece: Piece) {
        self.board[pos.to_index()] = piece;
    }
    pub fn get(&self, loc: &Pos2d) -> Piece {
        self.board[loc.to_index()]
    }
}