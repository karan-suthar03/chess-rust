#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color{
    White,
    Black,
}

impl Color{
    pub fn flip(self) -> Self {
        match self { 
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Hash)]
pub enum Piece{
    None,
    King(Color),
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
}

use std::fmt;

impl Piece {
    pub fn color(&self) -> Option<Color> {
        match self {
            Piece::King(c)
            | Piece::Queen(c)
            | Piece::Rook(c)
            | Piece::Bishop(c)
            | Piece::Knight(c)
            | Piece::Pawn(c) => Some(*c),
            Piece::None => None,
        }
    }

    pub fn is_white(&self) -> bool {
        matches!(self.color(), Some(Color::White))
    }
    
    pub fn is_piece(&self) -> bool {
        !self.color().is_none()
    }

}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Piece::Pawn(Color::White) => "P",
            Piece::Knight(Color::White) => "N",
            Piece::Bishop(Color::White) => "B",
            Piece::Rook(Color::White) => "R",
            Piece::Queen(Color::White) => "Q",
            Piece::King(Color::White) => "K",
            Piece::None => ".",
            Piece::Pawn(Color::Black) => "p",
            Piece::Knight(Color::Black) => "n",
            Piece::Bishop(Color::Black) => "b",
            Piece::Rook(Color::Black) => "r",
            Piece::Queen(Color::Black) => "q",
            Piece::King(Color::Black) => "k",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Pos2d {
    pub(crate) rank:u8,
    pub(crate) file:u8
}

impl Pos2d {
    pub fn from_index(index: usize) -> Pos2d {
        let rank = (index / 8) as u8;
        let file = (index % 8) as u8;
        Pos2d {
            rank,
            file
        }
    }

    pub fn to_index(&self) -> usize{
        (self.file + self.rank*8) as usize
    }
}

